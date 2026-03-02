use crate::client::TodoistClient;
use crate::error::ApiError;
use crate::models::{Collaborator, Project};
use crate::requests::projects::*;

impl TodoistClient {
    pub async fn get_all_projects(&self) -> Result<Vec<Project>, ApiError> {
        self.get_all_pages("/projects").await
    }

    pub async fn get_project(&self, id: &str) -> Result<Project, ApiError> {
        self.get(&format!("/projects/{id}")).await
    }

    pub async fn add_project(&self, req: &CreateProjectRequest) -> Result<Project, ApiError> {
        self.post("/projects", req).await
    }

    pub async fn update_project(
        &self,
        id: &str,
        req: &UpdateProjectRequest,
    ) -> Result<Project, ApiError> {
        self.post(&format!("/projects/{id}"), req).await
    }

    pub async fn delete_project(&self, id: &str) -> Result<(), ApiError> {
        self.delete(&format!("/projects/{id}")).await
    }

    pub async fn archive_project(&self, id: &str) -> Result<(), ApiError> {
        self.post_no_body(&format!("/projects/{id}/archive")).await
    }

    pub async fn unarchive_project(&self, id: &str) -> Result<(), ApiError> {
        self.post_no_body(&format!("/projects/{id}/unarchive"))
            .await
    }

    pub async fn get_project_collaborators(
        &self,
        project_id: &str,
    ) -> Result<Vec<Collaborator>, ApiError> {
        self.get(&format!("/projects/{project_id}/collaborators"))
            .await
    }
}
