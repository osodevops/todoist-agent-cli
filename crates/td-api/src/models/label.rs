use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub order: Option<i32>,
    #[serde(default)]
    pub is_favorite: bool,
}
