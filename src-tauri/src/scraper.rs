use dotenv::dotenv;
use std::error::Error;
use thirtyfour::prelude::*;

use crate::database::schema::mansions;
use crate::links::extract_savills_urls;

use crate::scrape::scrape::{scrape_mansion, setup_driver, Mansionee};

#[tokio::main]
pub async fn test_scrape_mansions(links: Vec<String>) -> WebDriverResult<()> {
    delete_all_imgs("images");
    let driver = setup_driver("http://localhost:44444".to_string()).await;
    let _ = scrape_mansions(&driver, links).await;
    driver.quit().await?;
    Ok(())
}

#[tokio::main]
pub async fn test_massive_scrape() -> WebDriverResult<()> {
    delete_all_imgs("images");
    let driver = setup_driver("http://localhost:44444".to_string()).await;

    let file_path = "src/bookmarks.html";
    let links = extract_savills_urls(file_path);

    match links {
        Ok(urls) => {
            let _ = scrape_mansions(&driver, urls).await;
        }
        Err(e) => {
            eprintln!("Invalid URLs: {}", e);
        }
    }
    driver.quit().await?;
    Ok(()) // full run with current addresses took 5m47s
}

pub async fn scrape_mansions(
    driver: &WebDriver,
    urls: Vec<String>,
) -> Result<Vec<Mansionee>, Box<dyn Error + Send + Sync>> {
    let mut mansions: Vec<Mansionee> = Vec::new();
    for url in urls.iter() {
        match scrape_mansion(driver, url.to_string()).await {
            Ok(mansion) => {
                mansions.push(mansion);
            }
            Err(e) => println!("failed to scrape mansion <{url}>: {}", e),
        };
    }
    Ok(mansions)
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
