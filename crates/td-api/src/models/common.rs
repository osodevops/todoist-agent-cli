use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DueDate {
    pub date: String,
    #[serde(default)]
    pub is_recurring: bool,
    pub string: Option<String>,
    pub datetime: Option<String>,
    pub timezone: Option<String>,
    pub lang: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deadline {
    pub date: String,
    pub lang: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Duration {
    pub amount: u32,
    pub unit: String,
}
