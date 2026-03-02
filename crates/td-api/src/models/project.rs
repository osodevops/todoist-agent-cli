use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub parent_id: Option<String>,
    pub order: Option<i32>,
    #[serde(default)]
    pub is_favorite: bool,
    #[serde(default)]
    pub is_inbox_project: bool,
    #[serde(default)]
    pub is_team_inbox: bool,
    pub view_style: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collaborator {
    pub id: String,
    pub name: String,
    pub email: String,
}
