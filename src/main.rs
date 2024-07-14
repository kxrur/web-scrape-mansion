use std::error::Error;
use std::fs::File;
use std::io::copy;
use std::path::Path;

use reqwest::Client;
use thirtyfour::prelude::*;
use tokio::time::timeout;
use webdriver_install::Driver;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    //Driver::Chrome.install();

    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    // Navigate to https://wikipedia.org.
    driver
        .goto("https://search.savills.com/property-detail/bvi71435")
        .await?;

    let elem_address_one = driver
        .find(By::ClassName("sv-property-intro__address-line-1"))
        .await?;
    let elem_address_two = driver
        .find(By::ClassName("sv-property-intro__address-line-2"))
        .await?;
    let full_address = format!(
        "{}, {}",
        elem_address_one.text().await?,
        elem_address_two.text().await?
    );
    println!("Address: {}", full_address);

    let elem_cookie_block = driver
        .find(By::ClassName("sv-cookie-management__banner-cta"))
        .await?;
    let elem_cookie_button = elem_cookie_block.find(By::Tag("button")).await?;
    elem_cookie_button.click().await?;
    println!("past cookie");

    let elem_image_gallery_block = driver
        .find(By::ClassName(
            "Gallerystyled__LeadGalleryContent-sc-h7kctk-1",
        ))
        .await?;
    elem_image_gallery_block.scroll_into_view().await?;
    println!("clicking");
    elem_image_gallery_block.click().await?;

    let elem_image_block = driver
        .find(By::ClassName(
            "FullGallerystyled__FullGalleryWrapper-sc-cye8ql-0",
        ))
        .await?;
    //elem_image_gallery_block.send_keys(Key::Right);
    eval_images(elem_image_block, true).await?;

    driver.quit().await?;

    Ok(())
}

async fn eval_images(
    elem_block: WebElement,
    download: bool,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let images = elem_block.find_all(By::Tag("img")).await?;

    for (index, img) in images.iter().enumerate() {
        if download {
            if let Some(src) = img.attr("src").await? {
                println!("Image {}: {}", index + 1, src);
                download_image(
                    &Client::new(),
                    &src,
                    &format!("img/image_{}.jpg", index + 1),
                )
                .await?;
            } else {
                println!("Image {}: No src attribute", index + 1);
            }
        }
    }
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
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut file = File::create(file_path)?;
    copy(&mut bytes.as_ref(), &mut file)?;

    println!("Downloaded: {}", file_path);

    Ok(())
}
