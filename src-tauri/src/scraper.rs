use thirtyfour::prelude::*;

use crate::database::mansion::Mansion;
use crate::links::extract_savills_urls;

use crate::scrape::errors::Error;
use crate::scrape::scrape::{scrape_mansion, setup_driver};

pub async fn test_scrape_mansions(links: Vec<String>) -> Result<Vec<Mansion>, Error> {
    let driver = setup_driver("http://localhost:44444".to_string()).await;
    let mansions = scrape_mansions(&driver, links).await;
    match driver.quit().await {
        Ok(_) => println!("Quit driver successfully"),
        Err(e) => println!("{}", e),
    };
    Ok(mansions)
}

pub async fn test_massive_scrape() -> Result<Vec<Mansion>, Error> {
    let driver = setup_driver("http://localhost:44444".to_string()).await;

    let file_path = "src/bookmarks.html";
    let links = extract_savills_urls(file_path);

    let mansions = scrape_mansions(&driver, links).await;
    match driver.quit().await {
        Ok(_) => println!("Quit driver successfully"),
        Err(e) => println!("{}", e),
    };
    Ok(mansions)
}

pub async fn scrape_mansions(driver: &WebDriver, urls: Vec<String>) -> Vec<Mansion> {
    let mut mansions: Vec<Mansion> = Vec::new();
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
pub async fn scrape_one_mansion(url: String) -> Result<Mansion, Error> {
    delete_all_imgs("images");
    let driver = setup_driver("http://localhost:44444".to_string()).await;
    let result = match scrape_mansion(&driver, url.to_string()).await {
        Ok(mansion) => Ok(mansion),
        Err(e) => Err(Error::Parsing(format!("Failed to scrape mansion: {}", e))),
    };

    match driver.quit().await {
        Ok(_) => println!("Quit driver successfully"),
        Err(e) => println!("{}", e),
    };
    result
}
