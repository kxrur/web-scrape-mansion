// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate dotenv_codegen;

mod database;
mod links;
mod scrape;
mod scraper;

use database::{
    models::{Mansionee, NewMansionee},
    postgresql::get_mansionees,
};
use links::extract_savills_urls;
use scrape::errors::Error;
use scraper::{scrape_one_mansion, test_massive_scrape, test_scrape_mansions};
use std::sync::Mutex;
use tauri::Manager;

#[derive(Default)]
struct AppState {
    mansions: Vec<Mansionee>, //FIXME: use a HashMap instead of a Vec
}
use serde::{Deserialize, Serialize};
use specta::Type;
use specta_typescript::Typescript;
use tauri_specta::{collect_commands, Builder};

#[derive(Serialize, Deserialize, Type)]
struct House {
    rooms: u32,
    name: String,
    floors: u32,
}
#[derive(Serialize, Deserialize, Type)]
struct Person {
    dream_rooms: u32,
    name: String,
    dream_floors: u32,
}

fn main() {
    let builder = Builder::<tauri::Wry>::new()
        // Then register them (separated by a comma)
        .commands(collect_commands![
            load_mansions,
            load_database_mansions,
            load_all_url_mansions,
            get_mansion_by_id,
            scrape_one_mansion
        ]);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn _greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
#[specta::specta] // < You must annotate your commands
async fn load_mansions(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Vec<NewMansionee>, Error> {
    let n = 3;

    match n {
        1 => test_scrape_mansions(vec![
            "https://search.savills.com/property-detail/gbedrseds230103".to_string(),
        ]),

        2 => test_scrape_mansions(vec![
            "https://search.savills.com/property-detail/gbedrseds230103".to_string(),
            "https://search.savills.com/property-detail/gbslaklai220042".to_string(),
            "https://search.savills.com/property-detail/gbslaklak200005".to_string(),
        ]),

        _ => {
            println!("No data is scraped");
            test_scrape_mansions(vec!["".to_string()])
        }
    }
    .await // Return the result directly

    // If you need to update the state with the scraped mansions:
    // let mut state = state.lock().unwrap();
    // state.mansions = result?;
    // Ok(state.mansions.clone())
}

#[tauri::command]
#[specta::specta] // < You must annotate your commands
async fn load_all_url_mansions() -> Result<Vec<NewMansionee>, Error> {
    test_massive_scrape().await
}

#[tauri::command]
#[specta::specta] // < You must annotate your commands
fn load_database_mansions(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Vec<Mansionee>, Error> {
    let mansions = get_mansionees().unwrap();
    let mut state = state.lock().unwrap();
    state.mansions = mansions;
    Ok(state.mansions.clone())
}

#[tauri::command]
#[specta::specta]
fn get_mansion_by_id(
    id: u32,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Mansionee, Error> {
    let state = state.lock().unwrap();
    match state.mansions.get(id as usize) {
        Some(mansion) => Ok(mansion.clone()),
        None => Err(Error::Parsing(format!(
            "did not found mansion with id: {}",
            id
        ))),
    }
}
