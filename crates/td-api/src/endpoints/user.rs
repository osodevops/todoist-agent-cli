use crate::client::TodoistClient;
use crate::error::ApiError;
use crate::models::User;

impl TodoistClient {
    pub async fn get_user(&self) -> Result<User, ApiError> {
        self.get("/user").await
    }
}
