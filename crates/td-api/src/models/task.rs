use serde::{Deserialize, Serialize};

use super::common::{Deadline, DueDate, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    #[serde(default)]
    pub user_id: Option<String>,
    pub project_id: String,
    #[serde(default)]
    pub section_id: Option<String>,
    #[serde(default)]
    pub parent_id: Option<String>,
    pub content: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub priority: i32,
    #[serde(default)]
    pub due: Option<DueDate>,
    #[serde(default)]
    pub deadline: Option<Deadline>,
    #[serde(default)]
    pub duration: Option<Duration>,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default, alias = "child_order")]
    pub order: Option<i32>,
    #[serde(default, alias = "responsible_uid")]
    pub assignee_id: Option<String>,
    #[serde(default, alias = "assigned_by_uid")]
    pub assigner_id: Option<String>,
    #[serde(default, alias = "checked")]
    pub is_completed: bool,
    #[serde(default, alias = "added_at", alias = "date_added")]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default, alias = "date_completed")]
    pub completed_at: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickAddResult {
    #[serde(flatten)]
    pub task: Task,
}
