use std::error::Error;
use std::thread;
use std::time::Duration;

use webdriver_install::Driver;

use thirtyfour::prelude::*;

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

    let elem_image_block = driver
        .find(By::ClassName("Carouselstyled__CarouselTrack-sc-n9ijei-1"))
        .await?;
    thread::sleep(Duration::from_secs(3));
    println!("now");
    thread::sleep(Duration::from_secs(1));
    elem_image_block.click().await?;
    println!("clicked?");

    let elem_galery_block=driver
        .find(By::ClassName(".FullGallerystyled__FullGalleryWrapper-sc-cye8ql-0 > div:nth-child(1) > div:nth-child(1)"))
    .await?;

    let images = elem_galery_block.find_all(By::Tag("img")).await?;
    for (index, img) in images.iter().enumerate() {
        let src = img.attr("src").await?;
        println!(
            "Image {}: {}",
            index + 1,
            src.unwrap_or("No src attribute".to_string())
        );
    }

    driver.quit().await?;

    Ok(())
}
