use crate::client::TodoistClient;
use crate::error::ApiError;
use crate::models::Task;
use crate::pagination::PaginatedResponse;
use crate::requests::tasks::*;

impl TodoistClient {
    /// Get a paginated list of tasks, optionally filtered.
    pub async fn get_tasks(
        &self,
        query: &GetTasksQuery,
    ) -> Result<PaginatedResponse<Task>, ApiError> {
        self.get_with_query("/tasks", query).await
    }

    /// Get all tasks (auto-paginate).
    pub async fn get_all_tasks(&self) -> Result<Vec<Task>, ApiError> {
        self.get_all_pages("/tasks").await
    }

    /// Get a single task by ID.
    pub async fn get_task(&self, id: &str) -> Result<Task, ApiError> {
        self.get(&format!("/tasks/{id}")).await
    }

    /// Create a new task.
    pub async fn add_task(&self, req: &CreateTaskRequest) -> Result<Task, ApiError> {
        self.post("/tasks", req).await
    }

    /// Update an existing task (POST, not PATCH — API v1 convention).
    pub async fn update_task(&self, id: &str, req: &UpdateTaskRequest) -> Result<Task, ApiError> {
        self.post(&format!("/tasks/{id}"), req).await
    }

    /// Delete a task.
    pub async fn delete_task(&self, id: &str) -> Result<(), ApiError> {
        self.delete(&format!("/tasks/{id}")).await
    }

    /// Close (complete) a task.
    pub async fn close_task(&self, id: &str) -> Result<(), ApiError> {
        self.post_no_body(&format!("/tasks/{id}/close")).await
    }

    /// Reopen a completed task.
    pub async fn reopen_task(&self, id: &str) -> Result<(), ApiError> {
        self.post_no_body(&format!("/tasks/{id}/reopen")).await
    }

    /// Move a task to a different project/section/parent.
    pub async fn move_task(&self, id: &str, req: &MoveTaskRequest) -> Result<Task, ApiError> {
        self.post(&format!("/tasks/{id}/move"), req).await
    }

    /// Quick add a task using natural language.
    pub async fn quick_add_task(&self, text: &str) -> Result<Task, ApiError> {
        self.post(
            "/tasks/quick",
            &QuickAddRequest {
                text: text.to_string(),
            },
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{bearer_token, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup() -> (MockServer, TodoistClient) {
        let server = MockServer::start().await;
        let client = TodoistClient::with_base_url("test-token", server.uri()).unwrap();
        (server, client)
    }

    fn sample_task_json() -> serde_json::Value {
        serde_json::json!({
            "id": "abc123",
            "projectId": "proj1",
            "sectionId": null,
            "parentId": null,
            "content": "Buy milk",
            "description": "",
            "priority": 4,
            "due": null,
            "deadline": null,
            "duration": null,
            "labels": [],
            "order": 1,
            "assigneeId": null,
            "assignerId": null,
            "isCompleted": false,
            "createdAt": "2026-03-01T10:00:00Z",
            "updatedAt": null,
            "completedAt": null,
            "url": "https://app.todoist.com/app/task/abc123"
        })
    }

    #[tokio::test]
    async fn test_get_task() {
        let (server, client) = setup().await;

        Mock::given(method("GET"))
            .and(path("/tasks/abc123"))
            .and(bearer_token("test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(sample_task_json()))
            .mount(&server)
            .await;

        let task = client.get_task("abc123").await.unwrap();
        assert_eq!(task.id, "abc123");
        assert_eq!(task.content, "Buy milk");
        assert_eq!(task.priority, 4);
    }

    #[tokio::test]
    async fn test_get_tasks_paginated() {
        let (server, client) = setup().await;

        Mock::given(method("GET"))
            .and(path("/tasks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "results": [sample_task_json()],
                "nextCursor": null
            })))
            .mount(&server)
            .await;

        let query = GetTasksQuery::default();
        let page = client.get_tasks(&query).await.unwrap();
        assert_eq!(page.results.len(), 1);
        assert!(page.next_cursor.is_none());
    }

    #[tokio::test]
    async fn test_add_task() {
        let (server, client) = setup().await;

        Mock::given(method("POST"))
            .and(path("/tasks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(sample_task_json()))
            .mount(&server)
            .await;

        let req = CreateTaskRequest {
            content: "Buy milk".into(),
            ..Default::default()
        };
        let task = client.add_task(&req).await.unwrap();
        assert_eq!(task.content, "Buy milk");
    }

    #[tokio::test]
    async fn test_update_task() {
        let (server, client) = setup().await;

        Mock::given(method("POST"))
            .and(path("/tasks/abc123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(sample_task_json()))
            .mount(&server)
            .await;

        let req = UpdateTaskRequest {
            content: Some("Buy milk".into()),
            ..Default::default()
        };
        let task = client.update_task("abc123", &req).await.unwrap();
        assert_eq!(task.id, "abc123");
    }

    #[tokio::test]
    async fn test_delete_task() {
        let (server, client) = setup().await;

        Mock::given(method("DELETE"))
            .and(path("/tasks/abc123"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        client.delete_task("abc123").await.unwrap();
    }

    #[tokio::test]
    async fn test_close_task() {
        let (server, client) = setup().await;

        Mock::given(method("POST"))
            .and(path("/tasks/abc123/close"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        client.close_task("abc123").await.unwrap();
    }

    #[tokio::test]
    async fn test_reopen_task() {
        let (server, client) = setup().await;

        Mock::given(method("POST"))
            .and(path("/tasks/abc123/reopen"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        client.reopen_task("abc123").await.unwrap();
    }

    #[tokio::test]
    async fn test_quick_add_task() {
        let (server, client) = setup().await;

        Mock::given(method("POST"))
            .and(path("/tasks/quick"))
            .respond_with(ResponseTemplate::new(200).set_body_json(sample_task_json()))
            .mount(&server)
            .await;

        let task = client
            .quick_add_task("Buy milk tomorrow #Shopping")
            .await
            .unwrap();
        assert_eq!(task.id, "abc123");
    }

    #[tokio::test]
    async fn test_auth_error() {
        let (server, client) = setup().await;

        Mock::given(method("GET"))
            .and(path("/tasks/x"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let err = client.get_task("x").await.unwrap_err();
        assert!(matches!(err, ApiError::Auth { .. }));
    }

    #[tokio::test]
    async fn test_not_found() {
        let (server, client) = setup().await;

        Mock::given(method("GET"))
            .and(path("/tasks/missing"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let err = client.get_task("missing").await.unwrap_err();
        assert!(matches!(err, ApiError::NotFound { .. }));
    }

    #[tokio::test]
    async fn test_get_all_pages() {
        let (server, client) = setup().await;

        // Page 1 with cursor
        Mock::given(method("GET"))
            .and(path("/tasks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "results": [sample_task_json()],
                "nextCursor": "cursor_abc"
            })))
            .up_to_n_times(1)
            .mount(&server)
            .await;

        // Page 2 (no more pages)
        Mock::given(method("GET"))
            .and(path("/tasks"))
            .and(query_param("cursor", "cursor_abc"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "results": [sample_task_json()],
                "nextCursor": null
            })))
            .mount(&server)
            .await;

        let tasks = client.get_all_tasks().await.unwrap();
        assert_eq!(tasks.len(), 2);
    }
}
