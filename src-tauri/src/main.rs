// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate dotenv_codegen;

mod links;
mod scrape;
mod scraper;
mod store;
use log::debug;
use store::bucket::{connect_bucket, write_to_bucket};
use tokio;

use crate::scraper::{massive_scrape, testing, testing_async};

#[tokio::main]
async fn main() {
    env_logger::init();
    let _ = connect_bucket("mansion-images").await;
    write_to_bucket("src/text.txt", "mansion-images").await;
    //let _ = massive_scrape();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, test])
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
