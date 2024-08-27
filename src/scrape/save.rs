use async_recursion::async_recursion;
use base64::decode;
use regex::Regex;
use reqwest::Client;
use std::error::Error;
use std::fs::File;
use std::io::copy;
use std::io::Write;
use std::path::Path;
use tokio::fs::try_exists;

pub async fn download_image(
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
    println!("Downloaded SRC URL: {}", file_path);

    Ok(())
}
//FIXME: this doesn't actually donwload the image (when opening the file downloaded as data a dot
//image is displayed in my browser)
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

    println!("Saved DATA URL: {}", file_path);

    Ok(())
}

#[async_recursion]
pub async fn recursive_rename(file_path: &str) -> String {
    let path = Path::new(file_path);

    if !file_exists(file_path).await {
        return file_path.to_string();
    }
    let extension = path
        .extension()
        .expect("no extension found")
        .to_str()
        .unwrap();
    let file_name = path.file_stem().expect("can't get stem").to_str().unwrap();
    let new_file_name;

    let re = Regex::new(r"(.*)\((?<number>[0-9]*)").unwrap();
    if let Some(caps) = re.captures(file_name) {
        new_file_name = format!(
            "{}{}",
            file_name,
            caps["number"]
                .parse::<i32>()
                .expect("failed to parse file number")
                + 1
        );
    } else {
        new_file_name = file_name.to_string() + "(1)";
    };
    let binding = path.with_file_name(new_file_name + "." + extension);
    let new_path = binding.to_str().expect("could not convert path to str");

    recursive_rename(new_path).await.to_string()
}

async fn file_exists(file_path: &str) -> bool {
    try_exists(file_path)
        .await
        .expect("couldn't check existance")
}
