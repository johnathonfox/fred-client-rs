use thiserror::Error;

/// Errors that can occur when using the FRED API client.
#[derive(Error, Debug)]
pub enum FredError {
    /// An HTTP request failed.
    #[error("API request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// The FRED API returned an error response.
    #[error("FRED API error {code}: {message}")]
    Api { code: u16, message: String },

    /// A request parameter was invalid.
    #[error("Invalid parameter: {0}")]
    Validation(String),

    /// No API key was provided.
    #[error("API key missing")]
    MissingApiKey,

    /// Failed to parse the API response.
    #[error("Failed to parse response: {0}")]
    Parse(String),

    /// A URL construction error occurred.
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),
}
