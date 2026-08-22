use chrono::NaiveDate;
use serde::Deserialize;

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
