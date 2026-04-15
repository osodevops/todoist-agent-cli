use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub id: String,
    pub project_id: String,
    pub name: String,
    #[serde(default, alias = "section_order")]
    pub order: Option<i32>,
}
