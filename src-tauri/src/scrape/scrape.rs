use std::thread;
use std::time::Duration;

use dirs::home_dir;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thirtyfour::prelude::*;

use crate::database::mansion::{save_mansion, Mansion, NewMansion};
use crate::database::models::{Mansionee, NewMansionee, Picture};
use crate::database::sqlite::save_mansionee;
use crate::scrape::{
    action::close_cookie,
    save::{download_image, save_data_url_as_image},
};

use super::action::close_chat;
use super::save::recursive_rename;
use diesel::prelude::*;

pub const ADDRESS1_CS: &str = "sv-property-intro__address-line-1";
pub const ADDRESS2_CS: &str = "sv-property-intro__address-line-2";
//pub const PRICE_CS: &str = "sv-property-price__wrap:nth-child(2) > span:nth-child(3)";
pub const PRICE_CS: &str = "sv-property-price__wrap:nth-child(2)";
pub const SIZE_CS: &str = "sv--size > span:nth-child(1)";
pub const ROOMS_CS: &str = "sv-property-intro-footer__group:nth-child(2)";
pub const TYPE_CS: &str = "sv-property-intro-footer__group:nth-child(1) > div:nth-child(1)";
pub const GALLERY_BLOCK: &str = "Gallerystyled__LeadGalleryContent-sc-h7kctk-1";
pub const GALLERY_IMG: &str = "FullGallerystyled__FullGalleryWrapper-sc-cye8ql-0";

pub async fn setup_driver(server_url: String) -> WebDriver {
    //command: chromedriver --port=44444  (need the chromium package)
    let caps = DesiredCapabilities::chrome();
    //caps.add_arg("--headless=new")?; // hide the browser
    WebDriver::new(server_url, caps)
        .await
        .expect("Failed to load driver")
}

pub async fn scrape_mansion(driver: &WebDriver, url: String) -> WebDriverResult<Mansion> {
    println!("{}", url);
    driver.goto(&url).await?;
    close_cookie(driver, &url).await;

    if let Ok(address1) = eval_address(driver, ADDRESS1_CS).await {
        let address2 = eval_address(driver, ADDRESS2_CS).await?;
        let full_address = format!("{} {}", address1, address2);
        println!("full address: {}", full_address);
        let price = match eval_price(driver).await {
            Ok(it) => Some(it),
            Err(_) => {
                println!("No price found");
                None
            }
        };
        let size = match eval_size(driver).await {
            Ok(it) => Some(it),
            Err(_e) => {
                println!("did not find size item");
                None
            }
        };

        let (bedrooms, bathrooms, receptions) = eval_room(driver).await?;

        let house_type = eval_type(driver).await?;

        let pictures: Vec<Picture> = {
            let mut result = None;
            for _ in 1..=10 {
                if let Some(pics) = eval_imgs(driver, &address1).await {
                    result = Some(pics);
                    break;
                }
                //tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
            result.unwrap_or_default()
        };
        let new_mansionee = NewMansionee::new(
            full_address,
            price,
            size,
            bedrooms,
            bathrooms,
            receptions,
            house_type,
        );
        new_mansionee.log();

        let new_mansion = NewMansion {
            mansion: new_mansionee,
            pictures,
        };

        let mansionee = save_mansion(new_mansion).unwrap();

        Ok(mansionee)
    } else {
        Err(WebDriverError::NotFound(url, "".to_string()))
    }
}

async fn eval_address(driver: &WebDriver, class_name: &str) -> Result<String, WebDriverError> {
    let elem_address = driver.find(By::ClassName(class_name)).await?;
    let address = elem_address.text().await?.split('\n').take(1).collect(); //TODO: find
                                                                            //better way to get the first line only
    println!("address: {}\n end", address);
    Ok(address)
}
async fn eval_price(driver: &WebDriver) -> Result<i32, WebDriverError> {
    let elem_price = driver.find(By::ClassName(PRICE_CS)).await?;
    let mut price_str = elem_price.text().await?;

    price_str.retain(|c| c != ','); //want an easier regex to deal with
    let re = Regex::new(r"(.*)(?<price>£[0-9].*)(.*)").unwrap();
    let caps = re.captures(&price_str).expect("no matches found");
    price_str = caps["price"].to_string();
    price_str.retain(|c| c.is_ascii_digit()); // remove trailing spaces / parentheses

    let price = match price_str.parse::<i32>() {
        Ok(it) => it,
        Err(_e) => {
            println!("failed on: {price_str}");
            -1
        }
    };

    println!("price: {}", price);
    Ok(price)
}

async fn eval_size(driver: &WebDriver) -> Result<f32, WebDriverError> {
    let elem_size = driver.find(By::ClassName(SIZE_CS)).await?;
    let mut size_str = elem_size.text().await?;
    size_str.retain(|c| c != ',');

    let re = Regex::new(r"(.*)\((?<size>[0-9\.]*)(.*)").unwrap();
    let caps = re.captures(&size_str).unwrap();
    size_str = caps["size"].to_string();

    let size: f32 = size_str.parse().unwrap();

    println!("size: {} sq m", size_str);
    Ok(size)
}

async fn eval_room(
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

async fn eval_type(driver: &WebDriver) -> Result<String, WebDriverError> {
    let elem_address = driver.find(By::ClassName(TYPE_CS)).await?;
    let house_type = elem_address.text().await?;

    println!("Type: {}", house_type);
    Ok(house_type)
}

async fn eval_imgs(driver: &WebDriver, address1: &String) -> Option<Vec<Picture>> {
    //FIXME: not sure if this is necessary
    close_chat(driver).await;
    let mut all_img_file_paths: Vec<Picture> = Vec::new();

    let elem_image_gallery_block = driver.find(By::ClassName(GALLERY_BLOCK)).await.ok()?;
    let _ = elem_image_gallery_block.click().await;

    let elem_image_block = driver.find(By::ClassName(GALLERY_IMG)).await.ok()?;
    let images = elem_image_block.find_all(By::Tag("img")).await.ok()?;

    for img in images.iter() {
        let alt_attr = img.attr("alt").await.ok()??;
        let mut name = alt_attr.split('-').collect::<Vec<&str>>()[0]
            .trim()
            .replace(' ', "_");
        if name.is_empty() {
            name = "_".to_string();
        }

        let src = img.attr("src").await.ok()??;

        // Don't store full path in DB!
        let database_path = home_dir()
            .map(|p| format!("{}/Desktop/images", p.to_string_lossy()))
            .unwrap_or_else(|| "./images".to_string());
        let foldername = remove_spaces(address1.clone());

        let file_path =
            recursive_rename(&format!("{}/{}/{}.jpg", database_path, foldername, &name)).await;

        let picture = Picture {
            path: format!("{}/{}/{}.jpg", database_path, foldername, &name),
            name,
        };
        all_img_file_paths.push(picture);

        let result = if src.starts_with("data:") {
            save_data_url_as_image(&src, &file_path).await
        } else {
            download_image(&Client::new(), &src, &file_path).await
        };

        if result.is_err() {
            eprintln!("Failed to save image: {}", src);
            continue; // optionally, or return None here if it's critical
        }
    }

    Some(all_img_file_paths)
}
fn remove_spaces(txt: String) -> String {
    txt.trim().replace(' ', "_")
}
