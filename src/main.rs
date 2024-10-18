use scraper::{Html, Selector};
// use serde_json::json;
use std::time::Duration;
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--no-sandbox")?;
    caps.add_chrome_arg("--disable-dev-shm-usage")?;
    caps.add_chrome_arg("--disable-gpu")?;
    caps.add_chrome_arg("--headless")?;

    // Update the WebDriver URL to use the correct address within the container
    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    let url = "https://www.uniqlo.com/uk/en/men/bottoms";
    driver.goto(url).await?;

    // Wait for the page to load
    driver
        .set_implicit_wait_timeout(Duration::from_secs(10))
        .await?;

    // Wait 5 seconds for the page to load
    tokio::time::sleep(Duration::from_secs(1)).await;

    let body = driver.source().await?;
    let document = Html::parse_document(&body);

    let product_tile_selector = Selector::parse(".fr-ec-product-tile-resize-wrapper").unwrap();

    println!("Searching for product tiles...");
    let product_tiles: Vec<_> = document.select(&product_tile_selector).collect();
    println!("Found {} product tiles", product_tiles.len());

    if product_tiles.is_empty() {
        println!("No product tiles found. The selector might be incorrect.");
        println!("First 1000 characters of the HTML:");
        println!("{}", &body[..1000.min(body.len())]);
    } else {
        let sale_items: Vec<_> = product_tiles
            .iter()
            .filter(|tile| {
                let text = tile.text().collect::<String>().to_lowercase();
                text.contains("sale")
            })
            .collect();

        println!("Found {} items on sale", sale_items.len());

        for product_tile in sale_items.iter() {
            // for (index, product_tile) in sale_items.iter().enumerate() {
            // println!("Sale Item #{}", index + 1);
            print_product_info(product_tile);
            println!();
        }
    }

    driver.quit().await?;
    Ok(())
}

fn print_product_info(product_tile: &scraper::ElementRef) {
    // let name_selector = Selector::parse(".fr-ec-product-tile__product-name").unwrap();

    let title_selector = Selector::parse("[data-testid='CoreTitle']").unwrap();

    let price_selector = Selector::parse(".fr-ec-price-text--color-promotional").unwrap();
    let original_price_selector = Selector::parse(".fr-ec-price__original-price").unwrap();

    if let Some(name) = product_tile.select(&title_selector).next() {
        println!("Name: {}", name.text().collect::<String>().trim());
    }

    if let Some(price) = product_tile.select(&price_selector).next() {
        println!("Price: {}", price.text().collect::<String>().trim());
    }

    if let Some(original_price) = product_tile.select(&original_price_selector).next() {
        println!(
            "Original Price: {}",
            original_price.text().collect::<String>().trim()
        );
    }
}
