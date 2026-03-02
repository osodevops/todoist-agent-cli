use crate::client::TodoistClient;
use crate::error::ApiError;
use crate::models::SavedFilter;
use crate::requests::filters::*;

impl TodoistClient {
    pub async fn get_all_filters(&self) -> Result<Vec<SavedFilter>, ApiError> {
        self.get("/filters").await
    }

    pub async fn get_filter(&self, id: &str) -> Result<SavedFilter, ApiError> {
        self.get(&format!("/filters/{id}")).await
    }

    pub async fn add_filter(&self, req: &CreateFilterRequest) -> Result<SavedFilter, ApiError> {
        self.post("/filters", req).await
    }

    pub async fn update_filter(
        &self,
        id: &str,
        req: &UpdateFilterRequest,
    ) -> Result<SavedFilter, ApiError> {
        self.post(&format!("/filters/{id}"), req).await
    }

    pub async fn delete_filter(&self, id: &str) -> Result<(), ApiError> {
        self.delete(&format!("/filters/{id}")).await
    }
}
