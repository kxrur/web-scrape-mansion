use dotenv::dotenv;
use std::error::Error;
use thirtyfour::prelude::*;
use tokio;

use crate::links::extract_savills_urls;

use crate::scrape::action::close_cookie;
use crate::scrape::scrape::{
    eval_address, eval_imgs, eval_price, eval_room, eval_size, eval_type, ADDRESS1_CS, ADDRESS2_CS,
};

#[tokio::main]
pub async fn testing_async() -> String {
    "async_imagees".to_string()
}

pub fn testing() -> String {
    "images".to_string()
}

#[tokio::main]
pub async fn massive_scrape() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv().ok();

    let file_path = "src/bookmarks.html";
    let all_links = extract_savills_urls(file_path);
    delete_all_imgs("images");

    //Driver::Chrome.install();

    //command: chromedriver --port=44444  (need the chromium package)

    match all_links {
        Ok(urls) => {
            let caps = DesiredCapabilities::chrome();
            //caps.add_arg("--headless=new")?; // hide the browser
            let driver = WebDriver::new("http://localhost:44444", caps)
                .await
                .expect("Failed to load driver");
            //FIXME: make sure url is unique (have duplicates in data)
            for (i, url) in urls.iter().enumerate() {
                println!("{}", url);
                driver.goto(url).await?;
                if i == 0 {
                    close_cookie(&driver, url).await;
                }

                if let Ok(address1) = eval_address(&driver, ADDRESS1_CS).await {
                    let address2 = eval_address(&driver, ADDRESS2_CS).await?;
                    let full_address = format!("{} {}", address1, address2);
                    println!("full address: {}", full_address);
                    let price = match eval_price(&driver).await {
                        Ok(it) => Some(it),
                        Err(_) => {
                            println!("No price found");
                            None
                        }
                    };
                    let size = match eval_size(&driver).await {
                        Ok(it) => Some(it),
                        Err(_e) => {
                            println!("did not find size item");
                            None
                        }
                    };

                    let (bedrooms, bathrooms, receptions) = eval_room(&driver).await?;

                    let house_type = eval_type(&driver).await?;

                    eval_imgs(&driver, &address1).await;
                } else {
                    println!("Mansion invalid for URL: {}", url);
                }
            }
            driver.quit().await?;
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
