use base64::decode;
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::copy;
use std::io::Write;
use std::path::Path;
use std::process;
use thirtyfour::prelude::*;
use tokio::time::timeout;
use webdriver_install::Driver;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let file_path = "bookmarks.html";
    let all_links = extract_savills_urls(file_path);
    // match extract_savills_urls(file_path) {
    //     Ok(urls) => {
    //         for url in urls {
    //             println!("{}", url);
    //         }
    //     }
    //     Err(e) => {
    //         eprintln!("Error: {}", e);
    //     }
    // }

    //Driver::Chrome.install();

    //command: chromedriver

    match all_links {
        Ok(urls) => {
            for url in urls {
                let caps = DesiredCapabilities::chrome();
                let driver = WebDriver::new("http://localhost:9515", caps).await?;
                println!("{}", &url);
                driver.goto(&url).await?;
                // driver
                //     .goto("https://search.savills.com/property-detail/bvi71435")
                //     .await?;

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

                driver.quit().await?;
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    Ok(())
}

async fn eval_images(
    elem_block: WebElement,
    download: bool,
    mut foldername: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    foldername.retain(|c| !c.is_whitespace());

    let images = elem_block.find_all(By::Tag("img")).await?;

    for (index, img) in images.iter().enumerate() {
        if download {
            if let Some(src) = img.attr("src").await? {
                if (src.starts_with("data:")) {
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
async fn save_data_url_as_image(
    data_url: &str,
    file_path: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Extract base64 data from data URL
    let base64_data = data_url
        .split(',')
        .nth(1)
        .ok_or("Invalid data URL format")?;
    let image_data = decode(base64_data)?;

    let mut file = File::create(file_path)?;
    file.write_all(&image_data)?;

    println!("Saved data URL image to: {}", file_path);

    Ok(())
}
async fn download_image(
    client: &Client,
    url: &str,
    file_path: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let response = client.get(url).send().await?;
    let bytes = response.bytes().await?;

    let path = Path::new(file_path);
    println!("attemptNewDir");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut file = File::create(file_path)?;
    println!("attemptCopy");
    copy(&mut bytes.as_ref(), &mut file)?;

    println!("Downloaded: {}", file_path);

    Ok(())
}
pub fn extract_savills_urls(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Load the HTML content from a file
    let html_content = fs::read_to_string(file_path)?;

    // Parse the HTML
    let document = Html::parse_document(&html_content);

    // Create a selector for the <a> tags
    let selector = Selector::parse("a").unwrap();

    // Regex to match URLs containing 'savills'
    let re = Regex::new(r"savills").unwrap();

    // Vector to store matching URLs
    let mut urls = Vec::new();

    // Iterate over the selected elements and collect matching URLs
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if re.is_match(href) {
                urls.push(href.to_string());
            }
        }
    }

    Ok(urls)
}
