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
