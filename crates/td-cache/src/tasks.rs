use crate::db::CacheDb;
use crate::error::CacheError;
use rusqlite::params;
use td_api::models::Task;

impl CacheDb {
    pub fn upsert_task(&self, task: &Task) -> Result<(), CacheError> {
        let due_json = task.due.as_ref().map(|d| serde_json::to_string(d).unwrap());
        let deadline_json = task
            .deadline
            .as_ref()
            .map(|d| serde_json::to_string(d).unwrap());
        let duration_json = task
            .duration
            .as_ref()
            .map(|d| serde_json::to_string(d).unwrap());
        let labels_json = serde_json::to_string(&task.labels)?;
        let raw_json = serde_json::to_string(task)?;

        self.conn().execute(
            "INSERT OR REPLACE INTO tasks (
                id, project_id, section_id, parent_id, content, description,
                priority, due_json, deadline_json, duration_json, labels_json,
                \"order\", assignee_id, assigner_id, is_completed,
                created_at, updated_at, completed_at, url, raw_json
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)",
            params![
                task.id,
                task.project_id,
                task.section_id,
                task.parent_id,
                task.content,
                task.description,
                task.priority,
                due_json,
                deadline_json,
                duration_json,
                labels_json,
                task.order,
                task.assignee_id,
                task.assigner_id,
                task.is_completed,
                task.created_at,
                task.updated_at,
                task.completed_at,
                task.url,
                raw_json,
            ],
        )?;
        Ok(())
    }

    pub fn upsert_tasks(&mut self, tasks: &[Task]) -> Result<(), CacheError> {
        let tx = self.conn_mut().transaction()?;
        for task in tasks {
            let due_json = task.due.as_ref().map(|d| serde_json::to_string(d).unwrap());
            let deadline_json = task
                .deadline
                .as_ref()
                .map(|d| serde_json::to_string(d).unwrap());
            let duration_json = task
                .duration
                .as_ref()
                .map(|d| serde_json::to_string(d).unwrap());
            let labels_json = serde_json::to_string(&task.labels)?;
            let raw_json = serde_json::to_string(task)?;

            tx.execute(
                "INSERT OR REPLACE INTO tasks (
                    id, project_id, section_id, parent_id, content, description,
                    priority, due_json, deadline_json, duration_json, labels_json,
                    \"order\", assignee_id, assigner_id, is_completed,
                    created_at, updated_at, completed_at, url, raw_json
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)",
                params![
                    task.id,
                    task.project_id,
                    task.section_id,
                    task.parent_id,
                    task.content,
                    task.description,
                    task.priority,
                    due_json,
                    deadline_json,
                    duration_json,
                    labels_json,
                    task.order,
                    task.assignee_id,
                    task.assigner_id,
                    task.is_completed,
                    task.created_at,
                    task.updated_at,
                    task.completed_at,
                    task.url,
                    raw_json,
                ],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn replace_all_tasks(&mut self, tasks: &[Task]) -> Result<(), CacheError> {
        let tx = self.conn_mut().transaction()?;
        tx.execute("DELETE FROM tasks", [])?;
        for task in tasks {
            let due_json = task.due.as_ref().map(|d| serde_json::to_string(d).unwrap());
            let deadline_json = task
                .deadline
                .as_ref()
                .map(|d| serde_json::to_string(d).unwrap());
            let duration_json = task
                .duration
                .as_ref()
                .map(|d| serde_json::to_string(d).unwrap());
            let labels_json = serde_json::to_string(&task.labels)?;
            let raw_json = serde_json::to_string(task)?;

            tx.execute(
                "INSERT INTO tasks (
                    id, project_id, section_id, parent_id, content, description,
                    priority, due_json, deadline_json, duration_json, labels_json,
                    \"order\", assignee_id, assigner_id, is_completed,
                    created_at, updated_at, completed_at, url, raw_json
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)",
                params![
                    task.id, task.project_id, task.section_id, task.parent_id,
                    task.content, task.description, task.priority,
                    due_json, deadline_json, duration_json, labels_json,
                    task.order, task.assignee_id, task.assigner_id, task.is_completed,
                    task.created_at, task.updated_at, task.completed_at, task.url, raw_json,
                ],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_task(&self, id: &str) -> Result<Task, CacheError> {
        let task = self
            .conn()
            .query_row(
                "SELECT raw_json FROM tasks WHERE id = ?1",
                params![id],
                |row| {
                    let json: String = row.get(0)?;
                    Ok(json)
                },
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => CacheError::NotFound(format!("task {id}")),
                other => CacheError::Database(other),
            })?;
        Ok(serde_json::from_str(&task)?)
    }

    pub fn get_all_cached_tasks(&self) -> Result<Vec<Task>, CacheError> {
        let mut stmt = self
            .conn()
            .prepare("SELECT raw_json FROM tasks WHERE is_completed = 0 ORDER BY \"order\" ASC")?;
        let tasks = stmt
            .query_map([], |row| {
                let json: String = row.get(0)?;
                Ok(json)
            })?
            .filter_map(|r| r.ok())
            .filter_map(|json| serde_json::from_str(&json).ok())
            .collect();
        Ok(tasks)
    }

    pub fn get_tasks_by_project(&self, project_id: &str) -> Result<Vec<Task>, CacheError> {
        let mut stmt = self.conn().prepare(
            "SELECT raw_json FROM tasks WHERE project_id = ?1 AND is_completed = 0 ORDER BY \"order\" ASC",
        )?;
        let tasks = stmt
            .query_map(params![project_id], |row| {
                let json: String = row.get(0)?;
                Ok(json)
            })?
            .filter_map(|r| r.ok())
            .filter_map(|json| serde_json::from_str(&json).ok())
            .collect();
        Ok(tasks)
    }

    pub fn get_tasks_by_label(&self, label: &str) -> Result<Vec<Task>, CacheError> {
        let pattern = format!("%\"{label}\"%");
        let mut stmt = self.conn().prepare(
            "SELECT raw_json FROM tasks WHERE labels_json LIKE ?1 AND is_completed = 0 ORDER BY \"order\" ASC",
        )?;
        let tasks = stmt
            .query_map(params![pattern], |row| {
                let json: String = row.get(0)?;
                Ok(json)
            })?
            .filter_map(|r| r.ok())
            .filter_map(|json| serde_json::from_str(&json).ok())
            .collect();
        Ok(tasks)
    }

    pub fn get_tasks_due_today_or_overdue(&self, today: &str) -> Result<Vec<Task>, CacheError> {
        let mut stmt = self.conn().prepare(
            "SELECT raw_json FROM tasks
             WHERE is_completed = 0
             AND due_json IS NOT NULL
             AND json_extract(due_json, '$.date') <= ?1
             ORDER BY json_extract(due_json, '$.date') ASC, priority ASC",
        )?;
        let tasks = stmt
            .query_map(params![today], |row| {
                let json: String = row.get(0)?;
                Ok(json)
            })?
            .filter_map(|r| r.ok())
            .filter_map(|json| serde_json::from_str(&json).ok())
            .collect();
        Ok(tasks)
    }

    pub fn delete_cached_task(&self, id: &str) -> Result<(), CacheError> {
        self.conn()
            .execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn mark_task_completed(&self, id: &str) -> Result<(), CacheError> {
        self.conn().execute(
            "UPDATE tasks SET is_completed = 1 WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    pub fn mark_task_reopened(&self, id: &str) -> Result<(), CacheError> {
        self.conn().execute(
            "UPDATE tasks SET is_completed = 0 WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    pub fn get_inbox_tasks(&self) -> Result<Vec<Task>, CacheError> {
        let mut stmt = self.conn().prepare(
            "SELECT t.raw_json FROM tasks t
             JOIN projects p ON t.project_id = p.id
             WHERE p.is_inbox_project = 1 AND t.is_completed = 0
             ORDER BY t.\"order\" ASC",
        )?;
        let tasks = stmt
            .query_map([], |row| {
                let json: String = row.get(0)?;
                Ok(json)
            })?
            .filter_map(|r| r.ok())
            .filter_map(|json| serde_json::from_str(&json).ok())
            .collect();
        Ok(tasks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use td_api::models::common::DueDate;

    fn sample_task(id: &str, content: &str) -> Task {
        Task {
            id: id.to_string(),
            user_id: None,
            project_id: "proj1".to_string(),
            section_id: None,
            parent_id: None,
            content: content.to_string(),
            description: String::new(),
            priority: 4,
            due: None,
            deadline: None,
            duration: None,
            labels: vec![],
            order: Some(1),
            assignee_id: None,
            assigner_id: None,
            is_completed: false,
            created_at: Some("2026-03-01T10:00:00Z".to_string()),
            updated_at: None,
            completed_at: None,
            url: None,
        }
    }

    #[test]
    fn test_upsert_and_get_task() {
        let db = CacheDb::open_in_memory().unwrap();
        let task = sample_task("t1", "Buy milk");
        db.upsert_task(&task).unwrap();

        let fetched = db.get_task("t1").unwrap();
        assert_eq!(fetched.content, "Buy milk");
    }

    #[test]
    fn test_get_all_cached_tasks() {
        let mut db = CacheDb::open_in_memory().unwrap();
        let tasks = vec![sample_task("t1", "Task 1"), sample_task("t2", "Task 2")];
        db.upsert_tasks(&tasks).unwrap();

        let all = db.get_all_cached_tasks().unwrap();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_replace_all_tasks() {
        let mut db = CacheDb::open_in_memory().unwrap();
        let tasks1 = vec![sample_task("t1", "Old"), sample_task("t2", "Old 2")];
        db.upsert_tasks(&tasks1).unwrap();

        let tasks2 = vec![sample_task("t3", "New")];
        db.replace_all_tasks(&tasks2).unwrap();

        let all = db.get_all_cached_tasks().unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].id, "t3");
    }

    #[test]
    fn test_delete_cached_task() {
        let db = CacheDb::open_in_memory().unwrap();
        db.upsert_task(&sample_task("t1", "Delete me")).unwrap();
        db.delete_cached_task("t1").unwrap();

        let result = db.get_task("t1");
        assert!(result.is_err());
    }

    #[test]
    fn test_mark_task_completed() {
        let db = CacheDb::open_in_memory().unwrap();
        db.upsert_task(&sample_task("t1", "Complete me")).unwrap();
        db.mark_task_completed("t1").unwrap();

        // Should not appear in active tasks
        let all = db.get_all_cached_tasks().unwrap();
        assert_eq!(all.len(), 0);
    }

    #[test]
    fn test_get_tasks_by_label() {
        let db = CacheDb::open_in_memory().unwrap();
        let mut task = sample_task("t1", "Labeled");
        task.labels = vec!["urgent".to_string()];
        db.upsert_task(&task).unwrap();

        let found = db.get_tasks_by_label("urgent").unwrap();
        assert_eq!(found.len(), 1);

        let not_found = db.get_tasks_by_label("missing").unwrap();
        assert_eq!(not_found.len(), 0);
    }

    #[test]
    fn test_get_tasks_due_today_or_overdue() {
        let db = CacheDb::open_in_memory().unwrap();
        let mut task = sample_task("t1", "Due today");
        task.due = Some(DueDate {
            date: "2026-03-02".to_string(),
            is_recurring: false,
            string: Some("today".to_string()),
            datetime: None,
            timezone: None,
            lang: None,
        });
        db.upsert_task(&task).unwrap();

        let mut future_task = sample_task("t2", "Due later");
        future_task.due = Some(DueDate {
            date: "2026-04-01".to_string(),
            is_recurring: false,
            string: None,
            datetime: None,
            timezone: None,
            lang: None,
        });
        db.upsert_task(&future_task).unwrap();

        let today = db.get_tasks_due_today_or_overdue("2026-03-02").unwrap();
        assert_eq!(today.len(), 1);
        assert_eq!(today[0].content, "Due today");
    }
}
