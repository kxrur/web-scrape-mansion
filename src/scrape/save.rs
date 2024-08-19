use base64::decode;
use reqwest::Client;
use std::error::Error;
use std::fs::File;
use std::io::copy;
use std::io::Write;
use std::path::Path;

pub async fn download_image(
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
pub async fn save_data_url_as_image(
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
