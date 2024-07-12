use std::error::Error;

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

    driver.quit().await?;

    Ok(())
}
