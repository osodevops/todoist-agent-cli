use crate::db::CacheDb;
use crate::error::CacheError;
use rusqlite::params;
use td_api::models::Comment;

impl CacheDb {
    pub fn upsert_comment(&self, comment: &Comment) -> Result<(), CacheError> {
        let attachment_json = comment
            .attachment
            .as_ref()
            .map(|a| serde_json::to_string(a).unwrap());
        let raw_json = serde_json::to_string(comment)?;
        self.conn().execute(
            "INSERT OR REPLACE INTO comments (id, task_id, project_id, content, posted_at, attachment_json, raw_json)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                comment.id,
                comment.task_id,
                comment.project_id,
                comment.content,
                comment.posted_at,
                attachment_json,
                raw_json,
            ],
        )?;
        Ok(())
    }

    pub fn get_comments_for_task(&self, task_id: &str) -> Result<Vec<Comment>, CacheError> {
        let mut stmt = self
            .conn()
            .prepare("SELECT raw_json FROM comments WHERE task_id = ?1 ORDER BY posted_at ASC")?;
        let comments = stmt
            .query_map(params![task_id], |row| {
                let json: String = row.get(0)?;
                Ok(json)
            })?
            .filter_map(|r| r.ok())
            .filter_map(|json| serde_json::from_str(&json).ok())
            .collect();
        Ok(comments)
    }

    pub fn delete_cached_comment(&self, id: &str) -> Result<(), CacheError> {
        self.conn()
            .execute("DELETE FROM comments WHERE id = ?1", params![id])?;
        Ok(())
    }
}
