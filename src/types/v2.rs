use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
#[serde(untagged)]
enum BoolOrString {
    Bool(bool),
    String(String),
}

fn de_bool_or_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match BoolOrString::deserialize(deserializer)? {
        BoolOrString::Bool(value) => Ok(value),
        BoolOrString::String(value) => match value.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(serde::de::Error::custom("expected true/false")),
        },
    }
}

/// A v2 release observation page.
#[derive(Debug, Clone, Deserialize)]
pub struct V2ReleaseObservations {
    /// Whether another page is available.
    #[serde(deserialize_with = "de_bool_or_string")]
    pub has_more: bool,
    /// Cursor for the next page, present when `has_more` is true.
    #[serde(default)]
    pub next_cursor: Option<String>,
    /// Release metadata.
    pub release: V2Release,
    /// Series observations in this page.
    pub series: Vec<V2Series>,
}

/// Release metadata returned by the v2 bulk endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct V2Release {
    /// Release ID.
    pub release_id: i32,
    /// Release name.
    pub name: String,
    /// Originating release URL.
    pub url: String,
    /// Agencies that publish the release.
    #[serde(default)]
    pub sources: Vec<V2Source>,
}

/// A source agency in a v2 release response.
#[derive(Debug, Clone, Deserialize)]
pub struct V2Source {
    /// Source name.
    pub name: String,
    /// Source URL.
    pub url: String,
    /// Source notes.
    #[serde(default)]
    pub notes: Option<String>,
}

/// A series in a v2 release observations page.
#[derive(Debug, Clone, Deserialize)]
pub struct V2Series {
    /// Series ID.
    pub series_id: String,
    /// Series title.
    pub title: String,
    /// Data frequency.
    pub frequency: String,
    /// Units.
    pub units: String,
    /// Seasonal adjustment.
    pub seasonal_adjustment: String,
    /// Last updated timestamp in UTC.
    pub last_updated: String,
    /// Copyright information.
    #[serde(default)]
    pub copyright_id: Option<String>,
    /// Series notes.
    #[serde(default)]
    pub notes: Option<String>,
    /// Observations for this series.
    #[serde(default)]
    pub observations: Vec<V2Observation>,
}

/// A single observation in a v2 release observations page.
#[derive(Debug, Clone, Deserialize)]
pub struct V2Observation {
    /// Observation date.
    pub date: NaiveDate,
    /// Observed value as a string; FRED uses `"."` for missing values.
    #[serde(default)]
    pub value: Option<String>,
}
