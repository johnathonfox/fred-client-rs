use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Deserialize)]
#[serde(untagged)]
enum IntOrString {
    Int(i32),
    String(String),
}

fn de_i32_or_string<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    match IntOrString::deserialize(deserializer)? {
        IntOrString::Int(value) => Ok(value),
        IntOrString::String(value) => value.parse::<i32>().map_err(serde::de::Error::custom),
    }
}

fn de_string_or_int<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    match IntOrString::deserialize(deserializer)? {
        IntOrString::Int(value) => Ok(value.to_string()),
        IntOrString::String(value) => Ok(value),
    }
}

/// A FRED release.
#[derive(Debug, Clone, Deserialize)]
pub struct Release {
    /// Release ID.
    pub id: i32,
    /// Release name.
    pub name: String,
    /// Release URL.
    #[serde(default)]
    pub url: Option<String>,
    /// Real-time availability start date.
    #[serde(default)]
    pub realtime_start: Option<NaiveDate>,
    /// Real-time availability end date.
    #[serde(default)]
    pub realtime_end: Option<NaiveDate>,
}

/// A list of releases.
#[derive(Debug, Clone, Deserialize)]
pub struct Releases {
    /// The releases.
    #[serde(rename = "releases")]
    pub items: Vec<Release>,
}

/// A release date.
#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseDate {
    /// Release ID.
    pub release_id: i32,
    /// Date.
    pub date: NaiveDate,
    /// Release name.
    #[serde(default)]
    pub name: Option<String>,
}

/// A list of release dates.
#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseDates {
    /// The release dates.
    #[serde(rename = "release_dates")]
    pub items: Vec<ReleaseDate>,
}

/// A release table tree returned by `fred/release/tables`.
#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseTables {
    /// Root element name.
    #[serde(default)]
    pub name: Option<String>,
    /// Root element ID. Omitted by FRED when the root element is implied.
    #[serde(default)]
    pub element_id: Option<i32>,
    /// Release ID. FRED returns this as a string at the root.
    #[serde(deserialize_with = "de_string_or_int")]
    pub release_id: String,
    /// Elements keyed by element ID.
    #[serde(default)]
    pub elements: HashMap<String, ReleaseTableElement>,
}

/// An element in a release table tree.
#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseTableElement {
    /// Element ID.
    pub element_id: i32,
    /// Release ID.
    #[serde(default, deserialize_with = "de_i32_or_string")]
    pub release_id: i32,
    /// Series ID for series-type elements.
    #[serde(default)]
    pub series_id: Option<String>,
    /// Parent element ID.
    #[serde(default)]
    pub parent_id: Option<i32>,
    /// Table line.
    #[serde(default)]
    pub line: Option<String>,
    /// Element type, usually `series` or `table`.
    #[serde(rename = "type")]
    #[serde(default)]
    pub element_type: Option<String>,
    /// Element name.
    #[serde(default)]
    pub name: Option<String>,
    /// Tree level.
    #[serde(default)]
    pub level: Option<String>,
    /// Child elements.
    #[serde(default)]
    pub children: Vec<ReleaseTableElement>,
    /// Observation value when `include_observation_values=true`.
    #[serde(default)]
    pub value: Option<String>,
    /// Observation date when `include_observation_values=true`.
    #[serde(default)]
    pub date: Option<NaiveDate>,
    /// External link, when present.
    #[serde(default)]
    pub link: Option<String>,
    /// Notes, when present.
    #[serde(default)]
    pub notes: Option<String>,
}
