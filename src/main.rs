#[macro_use]
extern crate dotenv_codegen;

use dotenv::dotenv;
use std::error::Error;
use thirtyfour::prelude::*;
use tokio;

mod links;
use crate::links::extract_savills_urls;

mod scrape;
use crate::scrape::action::close_cookie;
use crate::scrape::scrape::{
    eval_address, eval_imgs, eval_price, eval_room, eval_size, eval_type, ADDRESS1_CS, ADDRESS2_CS,
};

mod database;
use crate::database::sql::{establish_pool, pull, push, some_mansions, Mansion};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv().ok();
    let _pool = establish_pool();

    let file_path = "bookmarks.html";
    let all_links = extract_savills_urls(file_path);

    //Driver::Chrome.install();

    //command: chromedriver

    match all_links {
        Ok(urls) => {
            let caps = DesiredCapabilities::chrome();
            //caps.add_arg("--headless=new")?; // hide the browser
            let driver = WebDriver::new("http://localhost:43945", caps).await?;
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
            eprintln!("Error: {}", e);
        }
    }
    Ok(())
}
