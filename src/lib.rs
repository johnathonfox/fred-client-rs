//! # fred-client-rs
//!
//! Async Rust client for the [Federal Reserve Economic Data (FRED) API](https://fred.stlouisfed.org/docs/api/fred/).
//!
//! ## Quick Start
//!
//! ```no_run
//! use fred_client_rs::FredClient;
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let client = FredClient::builder()
//!     .api_key(std::env::var("FRED_API_KEY")?)
//!     .build()?;
//!
//! let series = client.series("UNRATE").await?;
//! println!("{}", series.title);
//! # Ok(())
//! # }
//! ```
//!
//! ## Features
//!
//! - `blocking` — Enable synchronous/blocking API client

pub mod client;
pub mod endpoints;
pub mod error;
pub mod params;
pub mod types;

pub use client::{FredClient, FredClientBuilder};
pub use error::FredError;

/// Result type alias for fred-client-rs operations.
pub type Result<T> = std::result::Result<T, FredError>;
