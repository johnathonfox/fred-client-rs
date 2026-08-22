use chrono::NaiveDate;
use fred_client_rs::params::ObservationParams;
use fred_client_rs::FredClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        std::env::var("FRED_API_KEY").expect("FRED_API_KEY environment variable must be set");

    let client = FredClient::builder().api_key(api_key).build()?;

    // Get observations for unemployment rate
    let obs = client
        .series_observations(
            "UNRATE",
            ObservationParams::new()
                .observation_start(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap())
                .observation_end(NaiveDate::from_ymd_opt(2023, 12, 1).unwrap()),
        )
        .await?;

    println!("Observations for UNRATE (2023):");
    for observation in &obs.items {
        if let Some(value) = &observation.value {
            println!("  {}: {}%", observation.date, value);
        }
    }

    Ok(())
}
