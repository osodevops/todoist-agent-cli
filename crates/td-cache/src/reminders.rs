use crate::db::CacheDb;
use crate::error::CacheError;
use rusqlite::params;
use td_api::models::Reminder;

impl CacheDb {
    pub fn upsert_reminder(&self, reminder: &Reminder) -> Result<(), CacheError> {
        let due_json = reminder
            .due
            .as_ref()
            .map(|d| serde_json::to_string(d).unwrap());
        let raw_json = serde_json::to_string(reminder)?;
        self.conn().execute(
            "INSERT OR REPLACE INTO reminders (id, item_id, type, due_json, minute_offset, raw_json)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                reminder.id,
                reminder.item_id,
                reminder.reminder_type,
                due_json,
                reminder.minute_offset,
                raw_json,
            ],
        )?;
        Ok(())
    }

    pub fn get_reminders_for_task(&self, task_id: &str) -> Result<Vec<Reminder>, CacheError> {
        let mut stmt = self
            .conn()
            .prepare("SELECT raw_json FROM reminders WHERE item_id = ?1")?;
        let reminders = stmt
            .query_map(params![task_id], |row| {
                let json: String = row.get(0)?;
                Ok(json)
            })?
            .filter_map(|r| r.ok())
            .filter_map(|json| serde_json::from_str(&json).ok())
            .collect();
        Ok(reminders)
    }

    pub fn delete_cached_reminder(&self, id: &str) -> Result<(), CacheError> {
        self.conn()
            .execute("DELETE FROM reminders WHERE id = ?1", params![id])?;
        Ok(())
    }
}
