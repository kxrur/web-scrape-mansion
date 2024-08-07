match all_links {
        Ok(urls) => {
            for url in urls {
                let caps = DesiredCapabilities::chrome();
                let driver = WebDriver::new("http://localhost:9515", caps).await?;
                driver.goto(&url).await?;

                // Attempt to find elements and handle absence gracefully
                if let Ok(elem_address_one) = driver.find(By::ClassName("sv-property-intro__address-line-1")).await {
                    if let Ok(elem_address_two) = driver.find(By::ClassName("sv-property-intro__address-line-2")).await {
                        let full_address = format!(
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

                if let Ok(elem_cookie_block) = driver.find(By::ClassName("sv-cookie-management__banner-cta")).await {
                    if let Ok(elem_cookie_button) = elem_cookie_block.find(By::Tag("button")).await {
                        elem_cookie_button.click().await?;
                        println!("Past cookie");
                    } else {
                        println!("Cookie button not found for URL: {}", url);
                    }
                } else {
                    println!("Cookie block not found for URL: {}", url);
                }

                if let Ok(elem_image_gallery_block) = driver.find(By::ClassName("Gallerystyled__LeadGalleryContent-sc-h7kctk-1")).await {
                    elem_image_gallery_block.scroll_into_view().await?;
                    println!("Clicking");
                    elem_image_gallery_block.click().await?;

                    if let Ok(elem_image_block) = driver.find(By::ClassName("FullGallerystyled__FullGalleryWrapper-sc-cye8ql-0")).await {
                        eval_images(elem_image_block, true, url.clone()).await?;
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
