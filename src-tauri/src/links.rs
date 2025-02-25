use regex::Regex;
use scraper::{Html, Selector};
use std::fs;

pub fn extract_savills_urls(file_path: &str) -> Vec<String> {
    // Load the HTML content from a file
    let html_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            return Vec::new(); // Return an empty vector on error
        }
    };

    // Parse the HTML
    let document = Html::parse_document(&html_content);

    // Create a selector for the <a> tags
    let selector = match Selector::parse("a") {
        Ok(selector) => selector,
        Err(e) => {
            eprintln!("Failed to parse selector: {}", e);
            return Vec::new(); // Return an empty vector on error
        }
    };

    // Regex to match URLs containing 'savills'
    let re = match Regex::new(r"savills") {
        Ok(re) => re,
        Err(e) => {
            eprintln!("Failed to compile regex: {}", e);
            return Vec::new(); // Return an empty vector on error
        }
    };

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

    urls
}
