use crate::client::TodoistClient;
use crate::error::ApiError;
use crate::models::ActivityEvent;
use crate::requests::activity::*;

impl TodoistClient {
    pub async fn get_activity(
        &self,
        query: &GetActivityQuery,
    ) -> Result<Vec<ActivityEvent>, ApiError> {
        self.get_with_query("/activity/events", query).await
    }
}
