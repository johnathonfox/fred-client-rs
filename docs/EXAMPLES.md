# Examples

## Basic Usage

```rust
use fred_client_rs::FredClient;

let client = FredClient::builder()
    .api_key(std::env::var("FRED_API_KEY")?)
    .build()?;

let series = client.series("UNRATE").await?;
println!("{}", series.title);
```

## Observations with Date Range

```rust
use fred_client_rs::params::ObservationParams;
use chrono::NaiveDate;

let obs = client
    .series_observations(
        "UNRATE",
        ObservationParams::new()
            .observation_start(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap())
            .observation_end(NaiveDate::from_ymd_opt(2023, 12, 1).unwrap()),
    )
    .await?;
```

## Search Series

```rust
use fred_client_rs::params::SearchParams;

let results = client
    .series_search("gdp", SearchParams::new().limit(10))
    .await?;
```

## Categories

```rust
let category = client.category(125).await?;
let children = client.category_children(125, QueryParams::new()).await?;
```

## Releases

```rust
let releases = client.releases(QueryParams::new().limit(5)).await?;
let release = client.release(53).await?;
```
