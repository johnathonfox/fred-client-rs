use serde::Deserialize;

/// A FRED category.
#[derive(Debug, Clone, Deserialize)]
pub struct Category {
    /// Category ID.
    pub id: i32,
    /// Category name.
    pub name: String,
    /// Parent category ID.
    pub parent_id: i32,
    /// Category notes.
    #[serde(default)]
    pub notes: Option<String>,
}

/// A list of categories.
#[derive(Debug, Clone, Deserialize)]
pub struct Categories {
    /// The categories.
    #[serde(rename = "categories")]
    pub items: Vec<Category>,
}
