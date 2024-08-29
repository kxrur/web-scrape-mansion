use regex::Regex;
use reqwest::Client;
use thirtyfour::prelude::*;

use crate::scrape::save::{download_image, save_data_url_as_image};

use super::save::recursive_rename;

pub const ADDRESS1_CS: &str = "sv-property-intro__address-line-1";
pub const ADDRESS2_CS: &str = "sv-property-intro__address-line-2";
pub const PRICE_CS: &str = "sv-property-price__wrap:nth-child(2) > span:nth-child(3)";
pub const SIZE_CS: &str = "sv--size > span:nth-child(1)";
pub const ROOMS_CS: &str = "sv-property-intro-footer__group:nth-child(2)";
pub const TYPE_CS: &str = "sv-property-intro-footer__group:nth-child(1) > div:nth-child(1)";

pub async fn eval_address(driver: &WebDriver, class_name: &str) -> Result<String, WebDriverError> {
    let elem_address = driver.find(By::ClassName(class_name)).await?;
    let address = elem_address.text().await?.split('\n').take(1).collect(); //TODO: find
                                                                            //better way to get the first line only
    println!("address: {}\n end", address);
    Ok(address)
}
//FIXME: check for price in pound sterling in the first entry first and then move to second
//test on this listing: https://search.savills.com/property-detail/gbsvrsses210267
pub async fn eval_price(driver: &WebDriver) -> Result<i32, WebDriverError> {
    let elem_price = driver.find(By::ClassName(PRICE_CS)).await?;
    let mut price_str = elem_price.text().await?;

    price_str.retain(|c| c != '£' && c != ',' && c != '(' && c != ')');

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

pub async fn eval_imgs(driver: &WebDriver, address1: &String) {
    let elem_image_gallery_block = driver
        .find(By::ClassName(
            "Gallerystyled__LeadGalleryContent-sc-h7kctk-1",
        ))
        .await
        .expect("failed to get the image gallery block");

    let _ = elem_image_gallery_block.click().await;

    let elem_image_block = driver
        .find(By::ClassName(
            "FullGallerystyled__FullGalleryWrapper-sc-cye8ql-0",
        ))
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
}
fn remove_spaces(txt: String) -> String {
    txt.trim().replace(' ', "_")
}
