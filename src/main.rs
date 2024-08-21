#[macro_use]
extern crate dotenv_codegen;

use dotenv::dotenv;
use std::error::Error;
use thirtyfour::prelude::*;
use tokio;

mod links;
use crate::links::extract_savills_urls;

mod scrape;
use crate::scrape::scrape::eval_images;

mod database;
use crate::database::sql::some_mansions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv().ok();
    let _ = some_mansions();
    std::process::exit(1);

    let file_path = "bookmarks.html";
    let all_links = extract_savills_urls(file_path);

    //Driver::Chrome.install();

    //command: chromedriver

    match all_links {
        Ok(urls) => {
            let mut caps = DesiredCapabilities::chrome();
            //caps.add_arg("--headless=new")?; // hide the browser
            let driver = WebDriver::new("http://localhost:9515", caps).await?;
            for url in urls {
                println!("{}", &url);
                driver.goto(&url).await?;

                // Attempt to find elements and handle absence gracefully
                let mut full_address = String::new();
                if let Ok(elem_address_one) = driver
                    .find(By::ClassName("sv-property-intro__address-line-1"))
                    .await
                {
                    if let Ok(elem_address_two) = driver
                        .find(By::ClassName("sv-property-intro__address-line-2"))
                        .await
                    {
                        full_address = format!(
                            "{}, {}",
                            elem_address_one.text().await?,
                            elem_address_two.text().await?
                        );
                        println!("Address: {}", full_address);
                    } else {
                        println!("Address line 2 not found for URL: {}", url);
                    }
                } else {
                    println!("Address line 1 not found for URL: {}", url);
                }

                if let Ok(elem_cookie_block) = driver
                    .find(By::ClassName("sv-cookie-management__banner-cta"))
                    .await
                {
                    if let Ok(elem_cookie_button) = elem_cookie_block.find(By::Tag("button")).await
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
            }
            driver.quit().await?;
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    Ok(())
}
