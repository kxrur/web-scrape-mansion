use reqwest::Client;
use std::error::Error;
use thirtyfour::prelude::*;

use crate::scrape::save::{download_image, save_data_url_as_image};

pub const ADDRESS1_CS: &str = "sv-property-intro__address-line-1";
pub const ADDRESS2_CS: &str = "sv-property-intro__address-line-2";
pub const PRICE_CS: &str = "sv-property-price__wrap:nth-child(2) > span:nth-child(3)";

pub async fn eval_images(
    elem_block: WebElement,
    download: bool,
    mut foldername: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    foldername.retain(|c| !c.is_whitespace());

    let images = elem_block.find_all(By::Tag("img")).await?;

    for (index, img) in images.iter().enumerate() {
        if download {
            if let Some(src) = img.attr("src").await? {
                let file_path = format!("some/{}/image_{}.jpg", foldername, index + 1); // Adjust file extension if needed
                if src.starts_with("data:") {
                    println!("Data URL found: {}", src);
                    save_data_url_as_image(&src, &file_path).await?;
                } else {
                    download_image(&Client::new(), &src, &file_path).await?;
                }
            } else {
                println!("Image {}: No src attribute", index + 1);
            }
        }
    }
    Ok(())
}
pub async fn eval_address(driver: &WebDriver, class_name: &str) -> Result<String, WebDriverError> {
    let elem_address_one = driver.find(By::ClassName(class_name)).await?;
    let address = elem_address_one.text().await?.split("\n").take(1).collect(); //TODO find
                                                                                //better way to get the first line only
    println!("address: {}\n end", address);
    Ok(address)
}
pub async fn eval_price(driver: &WebDriver, class_name: &str) -> Result<i32, WebDriverError> {
    let elem_price = driver.find(By::ClassName(class_name)).await?;
    let mut price_str = elem_price.text().await?;

    price_str.retain(|c| c != '£' && c != ',' && c != '(' && c != ')');

    let price: i32 = price_str.parse().unwrap();

    println!("price: {}\n end", price);
    Ok(price)
}
