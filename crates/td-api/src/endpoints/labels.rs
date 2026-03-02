use crate::client::TodoistClient;
use crate::error::ApiError;
use crate::models::Label;
use crate::requests::labels::*;

impl TodoistClient {
    pub async fn get_all_labels(&self) -> Result<Vec<Label>, ApiError> {
        self.get_all_pages("/labels").await
    }

    pub async fn get_label(&self, id: &str) -> Result<Label, ApiError> {
        self.get(&format!("/labels/{id}")).await
    }

    pub async fn add_label(&self, req: &CreateLabelRequest) -> Result<Label, ApiError> {
        self.post("/labels", req).await
    }

    pub async fn update_label(
        &self,
        id: &str,
        req: &UpdateLabelRequest,
    ) -> Result<Label, ApiError> {
        self.post(&format!("/labels/{id}"), req).await
    }

    pub async fn delete_label(&self, id: &str) -> Result<(), ApiError> {
        self.delete(&format!("/labels/{id}")).await
    }
}
