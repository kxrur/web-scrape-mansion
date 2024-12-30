// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate dotenv_codegen;

mod database;
mod links;
mod scrape;
mod scraper;

use crate::scraper::{testing, testing_async};
use std::sync::Mutex;
use tauri::Manager;

#[derive(Default)]
struct AppState {
    counter: u32,
}

fn main() {
    //let _ = massive_scrape();

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![increment_counter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn test() -> String {
    testing()
}

#[tauri::command]
async fn test_async() -> String {
    testing_async()
}

#[tauri::command]
fn increment_counter(state: tauri::State<'_, Mutex<AppState>>) -> u32 {
    let mut state = state.lock().unwrap();
    state.counter += 1;
    state.counter
}
