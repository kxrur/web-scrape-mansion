use regex::Regex;
use reqwest::Client;
use std::error::Error;
use thirtyfour::prelude::*;

use crate::scrape::save::{download_image, save_data_url_as_image};

pub const ADDRESS1_CS: &str = "sv-property-intro__address-line-1";
pub const ADDRESS2_CS: &str = "sv-property-intro__address-line-2";
pub const PRICE_CS: &str = "sv-property-price__wrap:nth-child(2) > span:nth-child(3)";
pub const SIZE_CS: &str = "sv--size > span:nth-child(1)";
pub const ROOMS_CS: &str = "sv-property-intro-footer__group:nth-child(2)";
pub const TYPE_CS: &str = "sv-property-intro-footer__group:nth-child(1) > div:nth-child(1)";

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
    let elem_address = driver.find(By::ClassName(class_name)).await?;
    let address = elem_address.text().await?.split("\n").take(1).collect(); //TODO: find
                                                                            //better way to get the first line only
    println!("address: {}\n end", address);
    Ok(address)
}
pub async fn eval_price(driver: &WebDriver) -> Result<i32, WebDriverError> {
    let elem_price = driver.find(By::ClassName(PRICE_CS)).await?;
    let mut price_str = elem_price.text().await?;

    price_str.retain(|c| c != '£' && c != ',' && c != '(' && c != ')');

    let price: i32 = price_str.parse().unwrap();

    println!("price: {}", price);
    Ok(price)
}

pub async fn eval_size(driver: &WebDriver) -> Result<f64, WebDriverError> {
    let elem_size = driver.find(By::ClassName(SIZE_CS)).await?;
    let mut size_str = elem_size.text().await?;
    size_str.retain(|c| c != ',');

    let re = Regex::new(r"(.*)\((?<size>[0-9\.]*)(.*)").unwrap();
    let caps = re.captures(&size_str).unwrap();
    size_str = caps["size"].to_string();

    let size: f64 = size_str.parse().unwrap();

    println!("size: {} sq m", size_str);
    Ok(size)
}

pub async fn eval_room(
    driver: &WebDriver,
) -> Result<(Option<i32>, Option<i32>, Option<i32>), WebDriverError> {
    let (mut room, mut bath, mut rec): (Option<i32>, Option<i32>, Option<i32>) = (None, None, None);
    let elem_room = driver.find_all(By::ClassName(ROOMS_CS)).await?;

    for roomie_cat in elem_room.iter() {
        let roomies = roomie_cat.find_all(By::Tag("div")).await?;

        for roomie in roomies {
            let roomie_span = roomie.find_all(By::Tag("span")).await?;
            let label = roomie_span[1].text().await?;
            let _: Option<i32> = match roomie_span[0].text().await?.parse() {
                Ok(it) => {
                    match label.as_str() {
                        "Bedrooms" => room = Some(it),
                        "Bathrooms" => bath = Some(it),
                        "Receptions" => rec = Some(it),
                        _ => (),
                    };
                    Some(it)
                }
                Err(e) => {
                    eprint!("{}", e);
                    None
                }
            };
        }
    }

    println!(
        "Bedrooms: {:?}\nBathrooms: {:?}\nReceptions: {:?}",
        room, bath, rec
    );
    Ok((room, bath, rec))
}

pub async fn eval_type(driver: &WebDriver) -> Result<String, WebDriverError> {
    let elem_address = driver.find(By::ClassName(TYPE_CS)).await?;
    let house_type = elem_address.text().await?;

    println!("Type: {}", house_type);
    Ok(house_type)
}
