// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate dotenv_codegen;

mod database;
mod links;
mod scrape;
mod scraper;

use database::{
    mansion::{get_mansions, Mansion},
    models::{Mansionee, NewMansionee, Setting},
    sqlite::{establish_connection, get_mansionees, get_settings, save_setting, update_setting},
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use scrape::errors::Error;
use scraper::{scrape_one_mansion, test_massive_scrape, test_scrape_mansions};
use std::sync::Mutex;
use tauri::Manager;

#[derive(Default)]
struct AppState {
    mansions: Vec<Mansion>, //FIXME: use a HashMap instead of a Vec
    settings: Vec<Setting>,
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

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

fn main() {
    let mut connection = establish_connection();

    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Error migrating");

    let builder = Builder::<tauri::Wry>::new()
        // Then register them (separated by a comma)
        .commands(collect_commands![
            load_mansions,
            load_database_mansions,
            load_all_url_mansions,
            get_mansion_by_id,
            scrape_one_mansion,
            get_settings,
            save_setting,
            get_setting_by_id,
            update_setting,
            get_store_mansions,
            add_mansion,
            get_mansion_state_id_by_database_id
        ]);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);
            app.manage(Mutex::new(AppState::default()));

            let _ = get_settings(app.state());
            let _ = load_database_mansions(app.state());

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
async fn add_mansion(
    state: tauri::State<'_, Mutex<AppState>>,
    url: String,
) -> Result<Mansion, Error> {
    match scrape_one_mansion(url.clone()).await {
        Ok(mansion) => {
            let mut state = state.lock().unwrap();
            state.mansions.push(mansion.clone());
            Ok(mansion)
        }
        Err(e) => {
            println!("failed to scrape mansion with url: {}", &url);
            Err(e)
        }
    }
}

#[tauri::command]
#[specta::specta] // < You must annotate your commands
async fn get_store_mansions(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Vec<Mansion>, Error> {
    let state = state.lock().unwrap();
    Ok(state.mansions.clone())
}

#[tauri::command]
#[specta::specta] // < You must annotate your commands
async fn load_mansions(state: tauri::State<'_, Mutex<AppState>>) -> Result<Vec<Mansion>, Error> {
    let n = 2;

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
async fn load_all_url_mansions() -> Result<Vec<Mansion>, Error> {
    test_massive_scrape().await
}

#[tauri::command]
#[specta::specta] // < You must annotate your commands
fn load_database_mansions(state: tauri::State<'_, Mutex<AppState>>) -> Result<Vec<Mansion>, Error> {
    let mansions = get_mansions().unwrap();
    let mut state = state.lock().unwrap();
    state.mansions = mansions;
    Ok(state.mansions.clone())
}

#[tauri::command]
#[specta::specta] // < You must annotate your commands
fn get_mansion_state_id_by_database_id(
    mansionee: Mansionee,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<i32, Error> {
    let state = state.lock().unwrap();
    for (index, mansion) in state.mansions.iter().enumerate() {
        if mansion.mansion.uuid == mansionee.uuid {
            return Ok(index as i32);
        }
    }
    Err(Error::Parsing(format!(
        "did not find mansion with database id: {} in the store",
        mansionee.id
    )))
}

#[tauri::command]
#[specta::specta]
fn get_mansion_by_id(id: u32, state: tauri::State<'_, Mutex<AppState>>) -> Result<Mansion, Error> {
    let state = state.lock().unwrap();
    match state.mansions.get(id as usize) {
        Some(mansion) => Ok(mansion.clone()),
        None => Err(Error::Parsing(format!(
            "did not find mansion with id: {}",
            id
        ))),
    }
}

#[tauri::command]
#[specta::specta]
fn get_setting_by_id(id: u32, state: tauri::State<'_, Mutex<AppState>>) -> Result<Setting, Error> {
    let state = state.lock().unwrap();
    match state.settings.get(id as usize) {
        Some(setting) => Ok(setting.clone()),
        None => Err(Error::Parsing(format!(
            "did not find setting with id: {}",
            id
        ))),
    }
}
