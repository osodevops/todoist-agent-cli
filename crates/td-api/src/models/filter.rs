use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedFilter {
    pub id: String,
    pub name: String,
    pub query: String,
    pub color: Option<String>,
    pub order: Option<i32>,
    #[serde(default)]
    pub is_favorite: bool,
}
