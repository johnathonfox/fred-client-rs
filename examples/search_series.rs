use fred_client_rs::params::SearchParams;
use fred_client_rs::FredClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        std::env::var("FRED_API_KEY").expect("FRED_API_KEY environment variable must be set");

    let client = FredClient::builder().api_key(api_key).build()?;

    // Search for series
    let results = client
        .series_search("unemployment rate", SearchParams::new().limit(5))
        .await?;

    println!("Found {} series:", results.items.len());
    for series in &results.items {
        println!("  {} - {}", series.id, series.title);
    }

    Ok(())
}
