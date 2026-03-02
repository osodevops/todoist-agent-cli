use crate::client::TodoistClient;
use crate::error::ApiError;
use crate::models::Section;
use crate::requests::sections::*;

impl TodoistClient {
    pub async fn get_all_sections(&self) -> Result<Vec<Section>, ApiError> {
        self.get_all_pages("/sections").await
    }

    pub async fn get_section(&self, id: &str) -> Result<Section, ApiError> {
        self.get(&format!("/sections/{id}")).await
    }

    pub async fn add_section(&self, req: &CreateSectionRequest) -> Result<Section, ApiError> {
        self.post("/sections", req).await
    }

    pub async fn update_section(
        &self,
        id: &str,
        req: &UpdateSectionRequest,
    ) -> Result<Section, ApiError> {
        self.post(&format!("/sections/{id}"), req).await
    }

    pub async fn delete_section(&self, id: &str) -> Result<(), ApiError> {
        self.delete(&format!("/sections/{id}")).await
    }
}
