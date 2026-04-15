use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    #[serde(default)]
    pub task_id: Option<String>,
    #[serde(default)]
    pub project_id: Option<String>,
    pub content: String,
    #[serde(default)]
    pub posted_at: Option<String>,
    #[serde(default)]
    pub attachment: Option<Attachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub file_name: Option<String>,
    pub file_type: Option<String>,
    pub file_url: Option<String>,
    pub resource_type: Option<String>,
}
