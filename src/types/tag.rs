use serde::Deserialize;

/// A FRED tag.
#[derive(Debug, Clone, Deserialize)]
pub struct Tag {
    /// Tag name.
    pub name: String,
    /// Group ID (`freq`, `gen`, `geo`, `geot`, `rls`, `seas`, `src`, or `cc`).
    #[serde(default)]
    pub group_id: Option<String>,
    /// Notes.
    #[serde(default)]
    pub notes: Option<String>,
    /// Created date.
    #[serde(default)]
    pub created: Option<String>,
    /// Popularity.
    #[serde(default)]
    pub popularity: Option<i32>,
    /// Number of series with this tag.
    #[serde(default)]
    pub series_count: Option<i32>,
}

/// A list of tags.
#[derive(Debug, Clone, Deserialize)]
pub struct Tags {
    /// The tags.
    #[serde(rename = "tags")]
    pub items: Vec<Tag>,
}
