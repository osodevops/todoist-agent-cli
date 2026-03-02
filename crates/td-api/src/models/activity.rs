use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityEvent {
    pub id: Option<String>,
    pub event_type: String,
    pub object_type: String,
    pub object_id: Option<String>,
    pub parent_project_id: Option<String>,
    pub parent_item_id: Option<String>,
    pub initiator_id: Option<String>,
    pub event_date: Option<String>,
    pub extra_data: Option<serde_json::Value>,
}
