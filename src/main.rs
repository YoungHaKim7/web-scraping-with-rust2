use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::str::FromStr;

fn main() {
    // Set your target URL here
    let url = "https://example.com";

    // Fetch the HTML content from the given URL
    let response = get(url).expect("Failed to fetch");
    let html_content = response.text().expect("Error reading response body");

    // Parse the HTML content with Scraper
    let document: Html = Html::parse_document(&html_content);

    // Use selectors to find specific elements and extract data
    let title_selector = Selector::parse("title").unwrap();
    let title = document
        .select(&title_selector)
        .next()
        .map(|node| node.text())
        .unwrap();

    println!("Page Title: {:?}", title);

    // You can also use selectors to extract other data as needed
}
