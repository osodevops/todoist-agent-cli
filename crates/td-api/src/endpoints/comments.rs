use crate::client::TodoistClient;
use crate::error::ApiError;
use crate::models::Comment;
use crate::requests::comments::*;

impl TodoistClient {
    pub async fn get_comments_for_task(&self, task_id: &str) -> Result<Vec<Comment>, ApiError> {
        let query = [("task_id", task_id)];
        self.get_with_query("/comments", &query).await
    }

    pub async fn get_comments_for_project(
        &self,
        project_id: &str,
    ) -> Result<Vec<Comment>, ApiError> {
        let query = [("project_id", project_id)];
        self.get_with_query("/comments", &query).await
    }

    pub async fn get_comment(&self, id: &str) -> Result<Comment, ApiError> {
        self.get(&format!("/comments/{id}")).await
    }

    pub async fn add_comment(&self, req: &CreateCommentRequest) -> Result<Comment, ApiError> {
        self.post("/comments", req).await
    }

    pub async fn update_comment(
        &self,
        id: &str,
        req: &UpdateCommentRequest,
    ) -> Result<Comment, ApiError> {
        self.post(&format!("/comments/{id}"), req).await
    }

    pub async fn delete_comment(&self, id: &str) -> Result<(), ApiError> {
        self.delete(&format!("/comments/{id}")).await
    }
}
