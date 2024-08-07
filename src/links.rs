pub mod links;

use regex::Regex;
use scraper::{Html, Selector};
use std::fs;

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
