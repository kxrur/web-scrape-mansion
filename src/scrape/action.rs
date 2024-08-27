use thirtyfour::{By, WebDriver, WebElement};

pub async fn close_cookie(driver: &WebDriver, url: &String) {
    if let Ok(elem_cookie_block) = driver
        .find(By::ClassName("sv-cookie-management__banner-cta"))
        .await
    {
        if let Ok(elem_cookie_button) = elem_cookie_block.find(By::Tag("button")).await {
            match elem_cookie_button.click().await {
                Ok(_) => println!("Clicked cookie close"),
                Err(e) => eprintln!("{}", e),
            };
        } else {
            println!("Cookie button not found for URL: {}", url);
        }
    } else {
        println!("Cookie block not found for URL: {}", url);
    }
}
