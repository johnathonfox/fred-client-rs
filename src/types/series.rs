use chrono::NaiveDate;
use serde::Deserialize;

/// A FRED series.
#[derive(Debug, Clone, Deserialize)]
pub struct Series {
    /// Series ID.
    pub id: String,
    /// Series title.
    pub title: String,
    /// Observation start date.
    #[serde(default)]
    pub observation_start: Option<NaiveDate>,
    /// Observation end date.
    #[serde(default)]
    pub observation_end: Option<NaiveDate>,
    /// Frequency.
    #[serde(default)]
    pub frequency: Option<String>,
    /// Frequency short name.
    #[serde(default)]
    pub frequency_short: Option<String>,
    /// Units.
    #[serde(default)]
    pub units: Option<String>,
    /// Units short name.
    #[serde(default)]
    pub units_short: Option<String>,
    /// Seasonal adjustment.
    #[serde(default)]
    pub seasonal_adjustment: Option<String>,
    /// Seasonal adjustment short name.
    #[serde(default)]
    pub seasonal_adjustment_short: Option<String>,
    /// Last updated.
    #[serde(default)]
    pub last_updated: Option<String>,
    /// Popularity.
    #[serde(default)]
    pub popularity: Option<i32>,
    /// Group popularity.
    #[serde(default)]
    pub group_popularity: Option<i32>,
    /// Notes.
    #[serde(default)]
    pub notes: Option<String>,
}

/// A list of series.
#[derive(Debug, Clone, Deserialize)]
pub struct SeriesList {
    /// The series.
    #[serde(rename = "seriess")]
    pub items: Vec<Series>,
}

/// Vintage dates for a series.
#[derive(Debug, Clone, Deserialize)]
pub struct VintageDates {
    /// Real-time start date.
    #[serde(default)]
    pub realtime_start: Option<NaiveDate>,
    /// Real-time end date.
    #[serde(default)]
    pub realtime_end: Option<NaiveDate>,
    /// Order by field.
    #[serde(default)]
    pub order_by: Option<String>,
    /// Sort order.
    #[serde(default)]
    pub sort_order: Option<String>,
    /// Total count.
    #[serde(default)]
    pub count: Option<u32>,
    /// Offset.
    #[serde(default)]
    pub offset: Option<u32>,
    /// Limit.
    #[serde(default)]
    pub limit: Option<u32>,
    /// Vintage dates.
    #[serde(rename = "vintage_dates")]
    pub items: Vec<NaiveDate>,
}
