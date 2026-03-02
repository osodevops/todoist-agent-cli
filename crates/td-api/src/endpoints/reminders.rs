use crate::client::TodoistClient;
use crate::error::ApiError;
use crate::models::Reminder;
use crate::requests::reminders::*;

impl TodoistClient {
    pub async fn get_reminders(&self) -> Result<Vec<Reminder>, ApiError> {
        self.get("/reminders").await
    }

    pub async fn add_reminder(&self, req: &CreateReminderRequest) -> Result<Reminder, ApiError> {
        self.post("/reminders", req).await
    }

    pub async fn delete_reminder(&self, id: &str) -> Result<(), ApiError> {
        self.delete(&format!("/reminders/{id}")).await
    }
}
