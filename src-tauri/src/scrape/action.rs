use thirtyfour::{By, WebDriver};

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

pub async fn close_chat(driver: &WebDriver) {
    if let Ok(elem_chat_block) = driver
        .find(By::ClassName("giosg-chat-button-message-preview-bubble"))
        .await
    {
        if let Ok(elem_chat_button) = elem_chat_block.find(By::Tag("button")).await {
            match elem_chat_button.click().await {
                Ok(_) => println!("Clicked chat close"),
                Err(e) => eprintln!("{}", e),
            };
        } else {
            println!("Chat button not found");
        }
    } else {
        println!("Chat block not found");
    }
}
