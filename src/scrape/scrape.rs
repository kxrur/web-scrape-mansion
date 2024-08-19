use reqwest::Client;
use std::error::Error;
use thirtyfour::prelude::*;

use crate::scrape::save::{download_image, save_data_url_as_image};

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
                if src.starts_with("data:") {
                    // Handle data URL
                    println!("Data URL found: {}", src);
                    let file_path = format!("image_{}.jpg", index + 1); // Adjust file extension if needed
                    save_data_url_as_image(
                        &src,
                        &format!("{}/image_{}.jpg", foldername, index + 1),
                    )
                    .await?;
                } else {
                    println!("Image {}: {}", index + 1, src);
                    download_image(
                        &Client::new(),
                        &src,
                        &format!("{}/image_{}.jpg", foldername, index + 1),
                    )
                    .await?;
                }
            } else {
                println!("Image {}: No src attribute", index + 1);
            }
        }
    }
    Ok(())
}
