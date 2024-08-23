#[macro_use]
extern crate dotenv_codegen;

use dotenv::dotenv;
use std::error::Error;
use thirtyfour::prelude::*;
use tokio;

mod links;
use crate::links::extract_savills_urls;

mod scrape;
use crate::scrape::scrape::{
    eval_address, eval_images, eval_price, eval_room, eval_size, ADDRESS1_CS, ADDRESS2_CS,
};

mod database;
use crate::database::sql::{establish_pool, pull, push, some_mansions};

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
            let driver = WebDriver::new("http://localhost:9515", caps).await?;
            for url in urls {
                println!("{}", &url);
                driver.goto(&url).await?;

                if let Ok(address1) = eval_address(&driver, ADDRESS1_CS).await {
                    let address2 = eval_address(&driver, ADDRESS2_CS).await?;
                    let full_address = format!("{} {}", address1, address2);
                    println!("full address: {}", full_address);
                    let _price = match eval_price(&driver).await {
                        Ok(it) => Some(it),
                        Err(_) => {
                            println!("No price found");
                            None
                        }
                    }; // is an Option<i32> so gotta extract when using
                    let _size = eval_size(&driver).await?;

                    let (_bedrooms, _bathrooms, _receptions) = eval_room(&driver).await?; //option,

                    if let Ok(elem_cookie_block) = driver
                        .find(By::ClassName("sv-cookie-management__banner-cta"))
                        .await
                    {
                        if let Ok(elem_cookie_button) =
                            elem_cookie_block.find(By::Tag("button")).await
                        {
                            elem_cookie_button.click().await?;
                            println!("Past cookie");
                        } else {
                            println!("Cookie button not found for URL: {}", url);
                        }
                    } else {
                        println!("Cookie block not found for URL: {}", url);
                    }

                    if let Ok(elem_image_gallery_block) = driver
                        .find(By::ClassName(
                            "Gallerystyled__LeadGalleryContent-sc-h7kctk-1",
                        ))
                        .await
                    {
                        elem_image_gallery_block.scroll_into_view().await?;
                        println!("Clicking");
                        elem_image_gallery_block.click().await?;

                        if let Ok(elem_image_block) = driver
                            .find(By::ClassName(
                                "FullGallerystyled__FullGalleryWrapper-sc-cye8ql-0",
                            ))
                            .await
                        {
                            eval_images(elem_image_block, true, full_address).await?;
                        } else {
                            println!("Image block not found for URL: {}", url);
                        }
                    } else {
                        println!("Image gallery block not found for URL: {}", url);
                    }
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
