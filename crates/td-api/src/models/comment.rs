use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: String,
    pub task_id: Option<String>,
    pub project_id: Option<String>,
    pub content: String,
    pub posted_at: Option<String>,
    pub attachment: Option<Attachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub file_name: Option<String>,
    pub file_type: Option<String>,
    pub file_url: Option<String>,
    pub resource_type: Option<String>,
}
