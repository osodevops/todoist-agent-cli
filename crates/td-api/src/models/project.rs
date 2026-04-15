use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub parent_id: Option<String>,
    #[serde(default, alias = "child_order")]
    pub order: Option<i32>,
    #[serde(default)]
    pub is_favorite: bool,
    #[serde(default)]
    pub is_inbox_project: bool,
    #[serde(default)]
    pub is_team_inbox: bool,
    #[serde(default)]
    pub view_style: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collaborator {
    pub id: String,
    pub name: String,
    pub email: String,
}
