use reqwest::get;
use scraper::{Selector, Html};

#[tokio::main]
async fn main() {
    let url = "https://quatvn.club/hang-nguyen-xuxu_miss_you-onlyfans-leak-3/";

    let response = get(url).await.expect("Failed to send request.");
    let html = response.text().await.expect("Failed to read response.");

    let document = Html::parse_document(&html);

    let teaster_button_selector = Selector::parse(".mace-gallery-teaser-button").expect("Failed to parse selector.");
    let tester_button = document.select(&teaster_button_selector).next().expect("Failed to find element.");

    println!("{}", tester_button.html());
}
