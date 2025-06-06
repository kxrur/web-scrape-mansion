#[cfg_attr(mobile, tauri::mobile_entry_point)]
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
    models::{Mansionee, Setting},
    sqlite::{get_settings, init_database, save_setting, update_setting},
};
use scrape::errors::Error;
use scraper::scrape_one_mansion;
use std::sync::Mutex;
use tauri::Manager;

use specta_typescript::Typescript;
use tauri_specta::{collect_commands, Builder};

#[derive(Default)]
struct AppState {
    mansions: Vec<Mansion>, //FIXME: use a HashMap instead of a Vec
    settings: Vec<Setting>,
}

pub fn run() {
    let builder = Builder::<tauri::Wry>::new()
        // Then register them (separated by a comma)
        .commands(collect_commands![
            load_database_mansions,
            //load_all_url_mansions,
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
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);
            app.manage(Mutex::new(AppState::default()));

            init_database(app.app_handle());

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
    let db_path = state
        .lock()
        .unwrap()
        .settings
        .first()
        .unwrap()
        .clone()
        .db_path
        .unwrap();
    match scrape_one_mansion(url.clone(), &db_path).await {
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

// #[tauri::command]
// #[specta::specta] // < You must annotate your commands
// async fn load_all_url_mansions(
//     state: tauri::State<'_, Mutex<AppState>>,
// ) -> Result<Vec<Mansion>, Error> {
//     let db_path = state
//         .lock()
//         .unwrap()
//         .settings
//         .get(0)
//         .unwrap()
//         .clone()
//         .db_path
//         .unwrap();
//     test_massive_scrape(&db_path).await
// }

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
    let statee = state.clone();
    get_settings(state);
    let state = statee.lock().unwrap();
    match state.settings.get(id as usize) {
        Some(setting) => Ok(setting.clone()),
        None => Err(Error::Parsing(format!(
            "did not find setting with id: {}",
            id
        ))),
    }
}
