use serde::{Deserialize, Serialize};

use super::common::{Deadline, DueDate, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    #[serde(default)]
    pub user_id: Option<String>,
    pub project_id: String,
    pub section_id: Option<String>,
    pub parent_id: Option<String>,
    pub content: String,
    #[serde(default)]
    pub description: String,
    pub priority: i32,
    pub due: Option<DueDate>,
    pub deadline: Option<Deadline>,
    pub duration: Option<Duration>,
    #[serde(default)]
    pub labels: Vec<String>,
    pub order: Option<i32>,
    pub assignee_id: Option<String>,
    pub assigner_id: Option<String>,
    #[serde(default)]
    pub is_completed: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub completed_at: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickAddResult {
    #[serde(flatten)]
    pub task: Task,
}
