// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate dotenv_codegen;

mod database;
mod links;
mod scrape;
mod scraper;

use database::models::Mansionee;
use scrape::errors::Error;
use scraper::{test_massive_scrape, test_scrape_mansions};
use std::sync::Mutex;
use tauri::Manager;

#[derive(Default)]
struct AppState {
    counter: u32,
    Mansions: Vec<Mansionee>,
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

#[tauri::command]
#[specta::specta] // < You must annotate your commands
fn hello_world(person: Person) -> House {
    House {
        rooms: person.dream_rooms,
        name: person.name,
        floors: person.dream_floors,
    }
}

fn main() {
    let builder = Builder::<tauri::Wry>::new()
        // Then register them (separated by a comma)
        .commands(collect_commands![
            hello_world,
            increment_counter,
            load_mansions
        ]);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
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
async fn load_mansions(state: tauri::State<'_, Mutex<AppState>>) -> Result<Vec<Mansionee>, Error> {
    let n = 1;
    let mansions = match n {
        1 => test_scrape_mansions(vec![
            "https://search.savills.com/property-detail/gbedrseds230103".to_string(),
        ]),

        2 => test_scrape_mansions(vec![
            "https://search.savills.com/property-detail/gbedrseds230103".to_string(),
            "https://search.savills.com/property-detail/gbslaklai220042".to_string(),
            "https://search.savills.com/property-detail/gbslaklak200005".to_string(),
        ]),
        // 3 => Some(test_massive_scrape()),
        _ => {
            println!("No data is scraped");
            test_scrape_mansions(vec!["".to_string()])
        }
    }
    .await;
    let mut state = state.lock().unwrap();
    state.Mansions = mansions?;
    Ok(state.Mansions.clone())
}

#[tauri::command]
#[specta::specta] // < You must annotate your commands
fn increment_counter(state: tauri::State<'_, Mutex<AppState>>) -> u32 {
    let mut state = state.lock().unwrap();
    state.counter += 1;
    state.counter
}
