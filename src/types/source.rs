use serde::Deserialize;

/// A FRED source.
#[derive(Debug, Clone, Deserialize)]
pub struct Source {
    /// Source ID.
    pub id: i32,
    /// Source name.
    pub name: String,
    /// Source URL.
    #[serde(default)]
    pub url: Option<String>,
}

/// A list of sources.
#[derive(Debug, Clone, Deserialize)]
pub struct Sources {
    /// The sources.
    #[serde(rename = "sources")]
    pub items: Vec<Source>,
}
