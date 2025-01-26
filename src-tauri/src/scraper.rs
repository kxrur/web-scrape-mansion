use dotenv::dotenv;
use std::error::Error;
use thirtyfour::prelude::*;

use crate::links::extract_savills_urls;

use crate::scrape::scrape::{scrape_mansion, setup_driver};

#[tokio::main]
pub async fn test_single_scrape() -> WebDriverResult<()> {
    delete_all_imgs("images");
    let driver = setup_driver("http://localhost:44444".to_string()).await;
    let _ = scrape_mansion(
        &driver,
        "https://search.savills.com/property-detail/gbedrseds230103".to_string(),
    )
    .await;
    driver.quit().await?;
    Ok(())
}

#[tokio::main]
pub async fn test_massive_scrape() -> WebDriverResult<()> {
    delete_all_imgs("images");
    let driver = setup_driver("http://localhost:44444".to_string()).await;
    let _ = massive_scrape(&driver).await;
    driver.quit().await?;
    Ok(())
}
pub async fn massive_scrape(driver: &WebDriver) -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv().ok();

    let file_path = "src/bookmarks.html";
    let all_links = extract_savills_urls(file_path);
    delete_all_imgs("images");

    //Driver::Chrome.install();

    //command: chromedriver --port=44444  (need the chromium package)

    match all_links {
        Ok(urls) => {
            for (url) in urls.iter() {
                let _ = scrape_mansion(driver, url.to_string()).await;
            }
        }
        Err(e) => {
            eprintln!("Invalid URLs: {}", e);
        }
    }
    Ok(()) // full run with current addresses took 5m47s
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
