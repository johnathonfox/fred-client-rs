use crate::types::common::*;
use chrono::NaiveDate;
use serde::Serialize;

/// Common query parameters for FRED API requests.
#[derive(Debug, Clone, Default, Serialize)]
pub struct QueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    realtime_start: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    realtime_end: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_order: Option<SortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_by: Option<String>,
}

impl QueryParams {
    /// Create new empty query parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the real-time start date.
    pub fn realtime_start(mut self, date: NaiveDate) -> Self {
        self.realtime_start = Some(date);
        self
    }

    /// Set the real-time end date.
    pub fn realtime_end(mut self, date: NaiveDate) -> Self {
        self.realtime_end = Some(date);
        self
    }

    /// Set the result limit. The maximum depends on the endpoint
    /// (1000 for most list endpoints, 10000 for release dates and vintage
    /// dates, 100000 for series observations).
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the result offset.
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Set the sort order.
    pub fn sort_order(mut self, order: SortOrder) -> Self {
        self.sort_order = Some(order);
        self
    }

    /// Set the order by field.
    pub fn order_by(mut self, field: impl Into<String>) -> Self {
        self.order_by = Some(field.into());
        self
    }
}

/// Parameters for series observations.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ObservationParams {
    #[serde(flatten)]
    base: QueryParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    observation_start: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    observation_end: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    units: Option<Units>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency: Option<Frequency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aggregation_method: Option<AggregationMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_type: Option<OutputType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vintage_dates: Option<String>,
}

impl ObservationParams {
    /// Create new empty observation parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the observation start date.
    pub fn observation_start(mut self, date: NaiveDate) -> Self {
        self.observation_start = Some(date);
        self
    }

    /// Set the observation end date.
    pub fn observation_end(mut self, date: NaiveDate) -> Self {
        self.observation_end = Some(date);
        self
    }

    /// Set the units.
    pub fn units(mut self, units: Units) -> Self {
        self.units = Some(units);
        self
    }

    /// Set the frequency.
    pub fn frequency(mut self, frequency: Frequency) -> Self {
        self.frequency = Some(frequency);
        self
    }

    /// Set the aggregation method.
    pub fn aggregation_method(mut self, method: AggregationMethod) -> Self {
        self.aggregation_method = Some(method);
        self
    }

    /// Set the output type.
    pub fn output_type(mut self, output_type: OutputType) -> Self {
        self.output_type = Some(output_type);
        self
    }

    /// Set vintage dates (comma-separated YYYY-MM-DD values).
    pub fn vintage_dates(mut self, dates: impl Into<String>) -> Self {
        self.vintage_dates = Some(dates.into());
        self
    }

    /// Set the base query parameters.
    pub fn base(mut self, base: QueryParams) -> Self {
        self.base = base;
        self
    }
}

/// Parameters for series search.
#[derive(Debug, Clone, Default, Serialize)]
pub struct SearchParams {
    #[serde(flatten)]
    base: QueryParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    search_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_names: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_tag_names: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_search_text: Option<String>,
}

impl SearchParams {
    /// Create new empty search parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the search type (`full_text` or `series_id`).
    pub fn search_type(mut self, search_type: impl Into<String>) -> Self {
        self.search_type = Some(search_type.into());
        self
    }

    /// Set tag names to filter by (semicolon-separated).
    pub fn tag_names(mut self, tags: impl Into<String>) -> Self {
        self.tag_names = Some(tags.into());
        self
    }

    /// Set tag names to exclude (semicolon-separated).
    pub fn exclude_tag_names(mut self, tags: impl Into<String>) -> Self {
        self.exclude_tag_names = Some(tags.into());
        self
    }

    /// Set the tag group filter (`freq`, `gen`, `geo`, `geot`, `rls`, `seas`, `src`).
    ///
    /// Only used by the `series/search/tags` and `series/search/related_tags` endpoints.
    pub fn tag_group_id(mut self, tag_group_id: impl Into<String>) -> Self {
        self.tag_group_id = Some(tag_group_id.into());
        self
    }

    /// Set the tag search text.
    ///
    /// Only used by the `series/search/tags` and `series/search/related_tags` endpoints.
    pub fn tag_search_text(mut self, text: impl Into<String>) -> Self {
        self.tag_search_text = Some(text.into());
        self
    }

    /// Set the result limit (max 1000 for search endpoints).
    pub fn limit(mut self, limit: u32) -> Self {
        self.base = self.base.limit(limit);
        self
    }

    /// Set the result offset.
    pub fn offset(mut self, offset: u32) -> Self {
        self.base = self.base.offset(offset);
        self
    }

    /// Set the base query parameters.
    pub fn base(mut self, base: QueryParams) -> Self {
        self.base = base;
        self
    }
}

/// Parameters for the v2 release observations bulk endpoint.
///
/// v2 uses cursor pagination instead of limit/offset: omit `next_cursor` on
/// the first request, then pass back the `next_cursor` value from each response
/// until `has_more` is false.
#[derive(Debug, Clone, Default, Serialize)]
pub struct V2ReleaseObservationsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_cursor: Option<String>,
}

impl V2ReleaseObservationsParams {
    /// Create new empty v2 parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the result limit (1-500000, default 500000).
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit.clamp(1, 500_000));
        self
    }

    /// Set the cursor from a previous response's `next_cursor`.
    pub fn next_cursor(mut self, cursor: impl Into<String>) -> Self {
        self.next_cursor = Some(cursor.into());
        self
    }
}
