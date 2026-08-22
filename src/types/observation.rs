use chrono::NaiveDate;
use serde::Deserialize;

/// A FRED series observation.
#[derive(Debug, Clone, Deserialize)]
pub struct Observation {
    /// Observation date.
    pub date: NaiveDate,
    /// Observation value.
    #[serde(default)]
    pub value: Option<String>,
    /// Real-time start date (for ALFRED vintage data).
    #[serde(default)]
    pub realtime_start: Option<NaiveDate>,
    /// Real-time end date (for ALFRED vintage data).
    #[serde(default)]
    pub realtime_end: Option<NaiveDate>,
}

/// A list of observations.
#[derive(Debug, Clone, Deserialize)]
pub struct Observations {
    /// The observations.
    #[serde(rename = "observations")]
    pub items: Vec<Observation>,
    /// Observation count.
    #[serde(default)]
    pub count: Option<i32>,
    /// Units.
    #[serde(default)]
    pub units: Option<String>,
    /// Frequency.
    #[serde(default)]
    pub frequency: Option<String>,
}
