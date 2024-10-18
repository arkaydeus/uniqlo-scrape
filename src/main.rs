use actix_web::{get, web, App, HttpServer, Responder};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;
use thirtyfour::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
struct SaleItem {
    name: String,
    price: String,
    original_price: String,
}

#[get("/trousers")]
async fn get_trousers() -> impl Responder {
    match scrape_uniqlo().await {
        Ok(sale_items) => {
            let json_output = json!({ "sale_items": sale_items });
            web::Json(json_output)
        }
        Err(e) => {
            eprintln!("Error scraping Uniqlo: {:?}", e);
            let json_output = json!({ "error": "Failed to scrape data" });
            web::Json(json_output)
        }
    }
}

async fn scrape_uniqlo() -> WebDriverResult<Vec<SaleItem>> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--no-sandbox")?;
    caps.add_chrome_arg("--disable-dev-shm-usage")?;
    caps.add_chrome_arg("--disable-gpu")?;
    caps.add_chrome_arg("--headless")?;

    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    let url = "https://www.uniqlo.com/uk/en/men/bottoms";
    driver.goto(url).await?;

    driver
        .set_implicit_wait_timeout(Duration::from_secs(10))
        .await?;

    tokio::time::sleep(Duration::from_secs(1)).await;

    let body = driver.source().await?;
    let document = Html::parse_document(&body);

    let product_tile_selector = Selector::parse(".fr-ec-product-tile-resize-wrapper").unwrap();

    println!("Searching for product tiles...");
    let product_tiles: Vec<_> = document.select(&product_tile_selector).collect();
    println!("Found {} product tiles", product_tiles.len());

    let sale_items: Vec<SaleItem> = if product_tiles.is_empty() {
        println!("No product tiles found. The selector might be incorrect.");
        println!("First 1000 characters of the HTML:");
        println!("{}", &body[..1000.min(body.len())]);
        vec![]
    } else {
        product_tiles
            .iter()
            .filter(|tile| {
                let text = tile.text().collect::<String>().to_lowercase();
                text.contains("sale")
            })
            .filter_map(|tile| extract_product_info(tile))
            .collect()
    };

    println!("Found {} items on sale", sale_items.len());

    driver.quit().await?;
    Ok(sale_items)
}

fn extract_product_info(product_tile: &scraper::ElementRef) -> Option<SaleItem> {
    let title_selector = Selector::parse("[data-testid='CoreTitle']").unwrap();
    let price_selector = Selector::parse(".fr-ec-price-text--color-promotional").unwrap();
    let original_price_selector = Selector::parse(".fr-ec-price__original-price").unwrap();

    let name = product_tile
        .select(&title_selector)
        .next()?
        .text()
        .collect::<String>()
        .trim()
        .to_string();

    let price = product_tile
        .select(&price_selector)
        .next()?
        .text()
        .collect::<String>()
        .trim()
        .to_string();

    let original_price = product_tile
        .select(&original_price_selector)
        .next()?
        .text()
        .collect::<String>()
        .trim()
        .to_string();

    Some(SaleItem {
        name,
        price,
        original_price,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_trousers))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
