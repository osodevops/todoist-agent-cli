use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default, alias = "item_order")]
    pub order: Option<i32>,
    #[serde(default)]
    pub is_favorite: bool,
}
