use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Sort order for API responses.
#[derive(Debug, Clone, Copy, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SortOrder {
    /// Ascending order.
    #[default]
    Asc,
    /// Descending order.
    Desc,
}

/// Frequency aggregation for series observations.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Frequency {
    /// Daily frequency.
    D,
    /// Weekly frequency.
    W,
    /// Biweekly frequency.
    BW,
    /// Monthly frequency.
    M,
    /// Quarterly frequency.
    Q,
    /// Semiannual frequency.
    SA,
    /// Annual frequency.
    A,
    /// Weekly ending Friday.
    Wef,
    /// Weekly ending Thursday.
    Weth,
    /// Weekly ending Wednesday.
    Wew,
    /// Weekly ending Tuesday.
    Wetu,
    /// Weekly ending Monday.
    Wem,
    /// Weekly ending Sunday.
    Wesu,
    /// Weekly ending Saturday.
    Wesa,
    /// Biweekly ending Wednesday.
    Bwew,
    /// Biweekly ending Monday.
    Bwem,
}

/// Units for series observations.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Units {
    /// Levels (no transformation).
    Lin,
    /// Change.
    Chg,
    /// Change from year ago.
    Ch1,
    /// Percent change.
    Pch,
    /// Percent change from year ago.
    Pc1,
    /// Compounded annual rate of change.
    Pca,
    /// Continuously compounded rate of change.
    Cch,
    /// Continuously compounded annual rate of change.
    Cca,
    /// Natural log.
    Log,
}

/// Seasonal adjustment.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SeasonalAdjustment {
    /// Seasonally adjusted.
    Sa,
    /// Not seasonally adjusted.
    Nsa,
}

/// Aggregation method.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AggregationMethod {
    /// Average.
    Avg,
    /// Sum.
    Sum,
    /// End of period.
    Eop,
}

/// Output type for observations.
#[derive(Debug, Clone, Copy)]
pub enum OutputType {
    /// Observations by real-time period.
    RealTimePeriod = 1,
    /// Observations by vintage date, all observations.
    VintageDateAll = 2,
    /// Observations by vintage date, new and revised observations only.
    VintageDateNewRevised = 3,
    /// Observations, initial release only.
    InitialReleaseOnly = 4,
}

impl Serialize for OutputType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

/// Vintage date for observations.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(untagged)]
pub enum VintageDate {
    /// A specific date.
    Date(NaiveDate),
    /// Use the most recent vintage.
    #[serde(rename = "last")]
    Last,
}

/// A paginated response wrapper.
#[derive(Debug, Clone, Deserialize)]
pub struct PaginatedResponse<T> {
    /// The response data.
    #[serde(flatten)]
    pub data: T,
    /// Pagination metadata.
    #[serde(default)]
    pub pagination: Option<Pagination>,
}

/// Pagination metadata.
#[derive(Debug, Clone, Deserialize)]
pub struct Pagination {
    /// Number of items per page.
    pub limit: u32,
    /// Offset from start.
    pub offset: u32,
    /// Total number of items available.
    pub count: u32,
    /// Total number of pages.
    pub pages: u32,
}

/// A date range for filtering.
#[derive(Debug, Clone)]
pub struct DateRange {
    /// Start date.
    pub start: NaiveDate,
    /// End date.
    pub end: NaiveDate,
}
