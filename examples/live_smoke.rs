use chrono::NaiveDate;
use fred_client_rs::params::{ObservationParams, QueryParams, SearchParams};
use fred_client_rs::{FredClient, FredError};

fn sanitize(err: &FredError) -> String {
    match err {
        FredError::Request(_) => "request error".to_string(),
        FredError::Api { code, .. } => format!("api error status {code}"),
        FredError::Validation(_) => "validation error".to_string(),
        FredError::MissingApiKey => "missing api key".to_string(),
        FredError::Parse(_) => "parse error".to_string(),
        FredError::Url(_) => "url error".to_string(),
    }
}

async fn report<T, F>(
    name: &str,
    result: fred_client_rs::Result<T>,
    failures: &mut usize,
    render: F,
) where
    F: FnOnce(&T) -> String,
{
    match result {
        Ok(value) => println!("ok: {name} ({})", render(&value)),
        Err(err) => {
            *failures += 1;
            eprintln!("fail: {name} ({})", sanitize(&err));
        }
    }
}

#[tokio::main]
async fn main() {
    let api_key = match std::env::var("FRED_API_KEY") {
        Ok(key) if !key.trim().is_empty() => key,
        _ => {
            eprintln!("FRED_API_KEY is not set");
            std::process::exit(2);
        }
    };

    let client = match FredClient::builder().api_key(api_key).build() {
        Ok(client) => client,
        Err(err) => {
            eprintln!("client build failed: {}", sanitize(&err));
            std::process::exit(1);
        }
    };

    let mut failures = 0;

    report(
        "series",
        client.series("UNRATE").await,
        &mut failures,
        |series| format!("title={:?}", series.title),
    )
    .await;

    report(
        "series_observations",
        client
            .series_observations(
                "UNRATE",
                ObservationParams::new()
                    .observation_start(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
                    .observation_end(NaiveDate::from_ymd_opt(2024, 3, 1).unwrap())
                    .base(QueryParams::new().limit(5)),
            )
            .await,
        &mut failures,
        |observations| format!("count={}", observations.items.len()),
    )
    .await;

    report(
        "category",
        client.category(125).await,
        &mut failures,
        |category| format!("name={:?}", category.name),
    )
    .await;

    report(
        "series_search",
        client
            .series_search("gdp", SearchParams::new().limit(1))
            .await,
        &mut failures,
        |results| format!("count={}", results.items.len()),
    )
    .await;

    if failures == 0 {
        println!("live smoke test passed");
    } else {
        eprintln!("live smoke test failed: {failures} request(s) failed");
        std::process::exit(1);
    }
}
