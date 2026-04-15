use serde::{Deserialize, Serialize};

use super::common::DueDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub id: String,
    pub item_id: String,
    #[serde(rename = "type")]
    pub reminder_type: Option<String>,
    pub due: Option<DueDate>,
    pub minute_offset: Option<i32>,
}
