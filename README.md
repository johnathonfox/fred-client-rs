# fred-client-rs

[![Crates.io](https://img.shields.io/crates/v/fred-client-rs)](https://crates.io/crates/fred-client-rs)
[![Docs.rs](https://docs.rs/fred-client-rs/badge.svg)](https://docs.rs/fred-client-rs)

Async Rust client for the [Federal Reserve Economic Data (FRED) API](https://fred.stlouisfed.org/docs/api/fred/).

## Features

- **Complete API Coverage**: All FRED API v1 endpoints (Categories, Releases, Series, Sources, Tags)
- **Type-Safe**: Strongly typed request parameters and responses
- **Async-First**: Built on `reqwest` and `tokio`
- **Ergonomic**: Builder patterns for complex queries
- **Well-Tested**: Mock-based integration tests with `wiremock`

## Installation

```toml
[dependencies]
fred-client-rs = "0.2"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use fred_client_rs::FredClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FredClient::builder()
        .api_key(std::env::var("FRED_API_KEY")?)
        .build()?;

    let series = client.series("UNRATE").await?;
    println!("{}", series.title);

    Ok(())
}
```

## API Key

Get a free API key at [https://fred.stlouisfed.org/docs/api/api_key.html](https://fred.stlouisfed.org/docs/api/api_key.html)

## Documentation

- [API Documentation](https://docs.rs/fred-client-rs)
- [Architecture](docs/ARCHITECTURE.md)
- [Examples](docs/EXAMPLES.md)
- [Contributing](docs/CONTRIBUTING.md)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
