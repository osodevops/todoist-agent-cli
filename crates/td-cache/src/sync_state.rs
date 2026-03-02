use crate::db::CacheDb;
use crate::error::CacheError;
use rusqlite::params;

impl CacheDb {
    pub fn set_sync_state(&self, key: &str, value: &str) -> Result<(), CacheError> {
        self.conn().execute(
            "INSERT OR REPLACE INTO sync_state (key, value, updated_at)
             VALUES (?1, ?2, datetime('now'))",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn get_sync_state(&self, key: &str) -> Result<Option<String>, CacheError> {
        let result: Result<String, _> = self.conn().query_row(
            "SELECT value FROM sync_state WHERE key = ?1",
            params![key],
            |row| row.get(0),
        );
        match result {
            Ok(v) => Ok(Some(v)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(CacheError::Database(e)),
        }
    }

    pub fn get_last_sync_time(&self) -> Result<Option<String>, CacheError> {
        self.get_sync_state("last_sync_at")
    }

    pub fn set_last_sync_time(&self, time: &str) -> Result<(), CacheError> {
        self.set_sync_state("last_sync_at", time)
    }

    pub fn get_cached_resource_counts(&self) -> Result<CacheStats, CacheError> {
        let tasks: i64 = self
            .conn()
            .query_row("SELECT COUNT(*) FROM tasks", [], |row| row.get(0))?;
        let projects: i64 = self
            .conn()
            .query_row("SELECT COUNT(*) FROM projects", [], |row| row.get(0))?;
        let sections: i64 = self
            .conn()
            .query_row("SELECT COUNT(*) FROM sections", [], |row| row.get(0))?;
        let labels: i64 = self
            .conn()
            .query_row("SELECT COUNT(*) FROM labels", [], |row| row.get(0))?;

        Ok(CacheStats {
            tasks,
            projects,
            sections,
            labels,
        })
    }
}

#[derive(Debug)]
pub struct CacheStats {
    pub tasks: i64,
    pub projects: i64,
    pub sections: i64,
    pub labels: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_state_roundtrip() {
        let db = CacheDb::open_in_memory().unwrap();
        assert!(db.get_sync_state("test_key").unwrap().is_none());

        db.set_sync_state("test_key", "test_value").unwrap();
        assert_eq!(
            db.get_sync_state("test_key").unwrap().unwrap(),
            "test_value"
        );
    }

    #[test]
    fn test_cache_stats() {
        let db = CacheDb::open_in_memory().unwrap();
        let stats = db.get_cached_resource_counts().unwrap();
        assert_eq!(stats.tasks, 0);
        assert_eq!(stats.projects, 0);
    }
}
