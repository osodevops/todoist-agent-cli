use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DueDate {
    pub date: String,
    #[serde(default)]
    pub is_recurring: bool,
    #[serde(default)]
    pub string: Option<String>,
    #[serde(default)]
    pub datetime: Option<String>,
    #[serde(default)]
    pub timezone: Option<String>,
    #[serde(default)]
    pub lang: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deadline {
    pub date: String,
    #[serde(default)]
    pub lang: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Duration {
    pub amount: u32,
    pub unit: String,
}
