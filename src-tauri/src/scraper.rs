use thirtyfour::prelude::*;

use crate::database::models::NewMansionee;
use crate::links::extract_savills_urls;

use crate::scrape::errors::Error;
use crate::scrape::scrape::{scrape_mansion, setup_driver};
use crate::{load_database_mansions, AppState};
use std::sync::Mutex;

pub async fn test_scrape_mansions(links: Vec<String>) -> Result<Vec<NewMansionee>, Error> {
    delete_all_imgs("images");
    let driver = setup_driver("http://localhost:44444".to_string()).await;
    let mansions = scrape_mansions(&driver, links).await;
    match driver.quit().await {
        Ok(_) => println!("Quit driver successfully"),
        Err(e) => println!("{}", e),
    };
    Ok(mansions)
}

pub async fn test_massive_scrape() -> Result<Vec<NewMansionee>, Error> {
    delete_all_imgs("images");
    let driver = setup_driver("http://localhost:44444".to_string()).await;
    let mut mansions: Vec<NewMansionee> = Vec::new();

    let file_path = "src/bookmarks.html";
    let links = extract_savills_urls(file_path);

    mansions = scrape_mansions(&driver, links).await;
    match driver.quit().await {
        Ok(_) => println!("Quit driver successfully"),
        Err(e) => println!("{}", e),
    };
    Ok(mansions)
}

pub async fn scrape_mansions(driver: &WebDriver, urls: Vec<String>) -> Vec<NewMansionee> {
    let mut mansions: Vec<NewMansionee> = Vec::new();
    for url in urls.iter() {
        match scrape_mansion(driver, url.to_string()).await {
            Ok(mansion) => {
                mansions.push(mansion);
            }
            Err(e) => println!("failed to scrape mansion <{url}>: {}", e),
        };
    }
    mansions
}
fn delete_all_imgs(path: &str) {
    match std::fs::exists(path) {
        Ok(_) => {
            let _ = std::fs::remove_dir_all(path);
            println!("Deleted images in <{path}> successfully")
        }
        Err(e) => println!("Failed to delete images in <{path}>: {}", e),
    };
}

#[tauri::command]
#[specta::specta] // < You must annotate your commands
pub async fn scrape_one_mansion(
    state: tauri::State<'_, Mutex<AppState>>,
    url: String,
) -> Result<NewMansionee, Error> {
    delete_all_imgs("images");
    let driver = setup_driver("http://localhost:44444".to_string()).await;
    let result = match scrape_mansion(&driver, url.to_string()).await {
        Ok(mansion) => {
            match load_database_mansions(state) {
                Ok(_) => (),
                Err(e) => println!("Failed to update database mansions: {}", e),
            };
            Ok(mansion)
        }
        Err(e) => Err(Error::Parsing(format!("Failed to scrape mansion: {}", e))),
    };

    match driver.quit().await {
        Ok(_) => println!("Quit driver successfully"),
        Err(e) => println!("{}", e),
    };
    result
}
