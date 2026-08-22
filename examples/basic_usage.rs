use fred_client_rs::FredClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        std::env::var("FRED_API_KEY").expect("FRED_API_KEY environment variable must be set");

    let client = FredClient::builder().api_key(api_key).build()?;

    // Get a series
    let series = client.series("UNRATE").await?;
    println!("Series: {} - {}", series.id, series.title);
    println!("Frequency: {:?}", series.frequency);
    println!("Units: {:?}", series.units);

    Ok(())
}
