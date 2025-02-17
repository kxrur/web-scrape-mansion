use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thirtyfour::prelude::*;

use crate::database::models::Mansionee;
use crate::database::postgresql::save_mansionee;
use crate::scrape::{
    action::close_cookie,
    save::{download_image, save_data_url_as_image},
};

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

#[derive(Serialize, Deserialize, Clone, Queryable)]
pub struct Picture {
    pub path: String,
    pub name: String,
}

impl Mansionee {
    pub fn new(
        address: String,
        price: Option<i32>,
        size: Option<f64>,
        bedrooms: Option<i32>,
        bathrooms: Option<i32>,
        receptions: Option<i32>,
        house_type: String,
        pictures: Vec<Picture>,
    ) -> Self {
        let pictures_json = serde_json::to_value(pictures).ok();
        Mansionee {
            address,
            price,
            size,
            bedrooms,
            bathrooms,
            receptions,
            house_type,
            pictures: pictures_json,
        }
    }
    pub fn log(&self) {
        println!(
            "Here is a mansion: 
            Address: {}
            Price: {} 
            Size: {} sqm
            Bedrooms: {} 
            Bathrooms: {} 
            Receptions: {} 
            Type: {}
            Pictures:{:?}",
            self.address,
            self.price.map_or("N/A".to_string(), |p| format!("${}", p)),
            self.size.map_or("N/A".to_string(), |s| format!("{:.2}", s)),
            self.bedrooms.map_or("N/A".to_string(), |b| b.to_string()),
            self.bathrooms.map_or("N/A".to_string(), |b| b.to_string()),
            self.receptions.map_or("N/A".to_string(), |r| r.to_string()),
            self.house_type,
            self.pictures
        );
    }
}

pub async fn setup_driver(server_url: String) -> WebDriver {
    //command: chromedriver --port=44444  (need the chromium package)
    let caps = DesiredCapabilities::chrome();
    //caps.add_arg("--headless=new")?; // hide the browser
    WebDriver::new(server_url, caps)
        .await
        .expect("Failed to load driver")
}

pub async fn scrape_mansion(driver: &WebDriver, url: String) -> WebDriverResult<(Mansionee)> {
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

        let pictures = eval_imgs(driver, &address1).await;
        let mansion = Mansionee::new(
            full_address,
            price,
            size,
            bedrooms,
            bathrooms,
            receptions,
            house_type,
            pictures,
        );
        mansion.log();

        let mansionee = save_mansionee(mansion.clone());
        mansionee.log();

        Ok(mansion)
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
        Ok(it) => Some(it),
        Err(_e) => {
            println!("failed on: {price_str}");
            None
        }
    }
    .unwrap();

    println!("price: {}", price);
    Ok(price)
}

async fn eval_size(driver: &WebDriver) -> Result<f64, WebDriverError> {
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

async fn eval_imgs(driver: &WebDriver, address1: &String) -> Vec<Picture> {
    let mut all_img_file_paths: Vec<Picture> = Vec::new();
    let elem_image_gallery_block = driver
        .find(By::ClassName(GALLERY_BLOCK))
        .await
        .expect("failed to get the image gallery block");

    let _ = elem_image_gallery_block.click().await;

    let elem_image_block = driver
        .find(By::ClassName(GALLERY_IMG))
        .await
        .expect("did not find the image block");
    let images = elem_image_block
        .find_all(By::Tag("img"))
        .await
        .expect("no 'img' tags where found in the image block");
    for img in images.iter() {
        let mut name = img
            .attr("alt")
            .await
            .expect("no alt attribute was found for image")
            .expect("alt message is wrong")
            .split('-')
            .collect::<Vec<&str>>()[0]
            .trim()
            .replace(' ', "_");
        if name.is_empty() {
            name = "_".to_string();
        }
        let src = img
            .attr("src")
            .await
            .expect("no src attr found")
            .expect("src attr is wrong");

        let foldername = remove_spaces(address1.clone());
        let file_path = recursive_rename(&format!("images/{}/{}.jpg", foldername, &name)).await;
        let picture = Picture {
            path: format!("images/{}/{}.jpg", foldername, &name),
            name,
        };
        all_img_file_paths.push(picture);

        if src.starts_with("data:") {
            println!("Data URL found: {}", src);
            save_data_url_as_image(&src, &file_path)
                .await
                .expect("data URL did not save");
        } else {
            download_image(&Client::new(), &src, &file_path)
                .await
                .expect("src URL did not save");
        }
    }
    all_img_file_paths
}
fn remove_spaces(txt: String) -> String {
    txt.trim().replace(' ', "_")
}
