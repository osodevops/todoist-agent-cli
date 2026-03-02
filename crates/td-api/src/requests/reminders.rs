use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateReminderRequest {
    pub item_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_datetime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minute_offset: Option<i32>,
}
