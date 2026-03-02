use crate::db::CacheDb;
use crate::error::CacheError;
use rusqlite::params;
use td_api::models::Collaborator;

impl CacheDb {
    pub fn upsert_collaborator(&self, collab: &Collaborator) -> Result<(), CacheError> {
        let raw_json = serde_json::to_string(collab)?;
        self.conn().execute(
            "INSERT OR REPLACE INTO collaborators (id, name, email, raw_json)
             VALUES (?1, ?2, ?3, ?4)",
            params![collab.id, collab.name, collab.email, raw_json],
        )?;
        Ok(())
    }

    pub fn get_all_collaborators(&self) -> Result<Vec<Collaborator>, CacheError> {
        let mut stmt = self
            .conn()
            .prepare("SELECT raw_json FROM collaborators ORDER BY name ASC")?;
        let collabs = stmt
            .query_map([], |row| {
                let json: String = row.get(0)?;
                Ok(json)
            })?
            .filter_map(|r| r.ok())
            .filter_map(|json| serde_json::from_str(&json).ok())
            .collect();
        Ok(collabs)
    }
}
