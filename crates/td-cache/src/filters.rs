use crate::db::CacheDb;
use crate::error::CacheError;
use rusqlite::params;
use td_api::models::SavedFilter;

impl CacheDb {
    pub fn upsert_filter(&self, filter: &SavedFilter) -> Result<(), CacheError> {
        let raw_json = serde_json::to_string(filter)?;
        self.conn().execute(
            "INSERT OR REPLACE INTO filters (id, name, query, color, \"order\", is_favorite, raw_json)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                filter.id,
                filter.name,
                filter.query,
                filter.color,
                filter.order,
                filter.is_favorite,
                raw_json,
            ],
        )?;
        Ok(())
    }

    pub fn get_all_filters(&self) -> Result<Vec<SavedFilter>, CacheError> {
        let mut stmt = self
            .conn()
            .prepare("SELECT raw_json FROM filters ORDER BY \"order\" ASC")?;
        let filters = stmt
            .query_map([], |row| {
                let json: String = row.get(0)?;
                Ok(json)
            })?
            .filter_map(|r| r.ok())
            .filter_map(|json| serde_json::from_str(&json).ok())
            .collect();
        Ok(filters)
    }

    pub fn get_filter(&self, id: &str) -> Result<SavedFilter, CacheError> {
        let json: String = self
            .conn()
            .query_row(
                "SELECT raw_json FROM filters WHERE id = ?1",
                params![id],
                |row| row.get(0),
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    CacheError::NotFound(format!("filter {id}"))
                }
                other => CacheError::Database(other),
            })?;
        Ok(serde_json::from_str(&json)?)
    }

    pub fn delete_cached_filter(&self, id: &str) -> Result<(), CacheError> {
        self.conn()
            .execute("DELETE FROM filters WHERE id = ?1", params![id])?;
        Ok(())
    }
}
