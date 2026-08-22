use crate::error::FredError;
use crate::Result;
use reqwest::{Client, RequestBuilder};
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "https://api.stlouisfed.org/fred";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// A client for the FRED API.
#[derive(Debug, Clone)]
pub struct FredClient {
    api_key: String,
    base_url: String,
    client: Client,
}

impl FredClient {
    /// Create a new client builder.
    pub fn builder() -> FredClientBuilder {
        FredClientBuilder::default()
    }

    /// Create a request builder with the base URL and API key.
    pub(crate) fn request(&self, path: &str) -> Result<RequestBuilder> {
        let url = format!("{}/{}", self.base_url.trim_end_matches('/'), path);
        Ok(self.client.get(&url).query(&[
            ("api_key", &self.api_key),
            ("file_type", &"json".to_string()),
        ]))
    }

    /// Create a request builder for v2 endpoints, which use bearer auth and
    /// `format=json` instead of v1's `api_key`/`file_type` query params.
    pub(crate) fn request_v2(&self, path: &str) -> RequestBuilder {
        let url = format!("{}/{}", self.base_url.trim_end_matches('/'), path);
        self.client
            .get(&url)
            .bearer_auth(&self.api_key)
            .query(&[("format", "json")])
    }

    /// Execute a request and deserialize the response.
    pub(crate) async fn execute<T: serde::de::DeserializeOwned>(
        &self,
        request: RequestBuilder,
    ) -> Result<T> {
        let response = request.send().await?;
        let status = response.status();
        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(FredError::Api {
                code: status.as_u16(),
                message: text,
            });
        }
        let data = response.json::<T>().await.map_err(|err| {
            if err.is_decode() {
                FredError::Parse(err.to_string())
            } else {
                FredError::Request(err)
            }
        })?;
        Ok(data)
    }
}

/// Builder for [`FredClient`].
#[derive(Debug, Default)]
pub struct FredClientBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    timeout: Option<Duration>,
}

impl FredClientBuilder {
    /// Set the API key.
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Set the base URL (default: `https://api.stlouisfed.org/fred`).
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set the request timeout (default: 30s).
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Build the client.
    pub fn build(self) -> Result<FredClient> {
        let api_key = self.api_key.ok_or(FredError::MissingApiKey)?;
        let base_url = self
            .base_url
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string());
        let timeout = self.timeout.unwrap_or(DEFAULT_TIMEOUT);

        let client = Client::builder().timeout(timeout).build()?;

        Ok(FredClient {
            api_key,
            base_url,
            client,
        })
    }
}
