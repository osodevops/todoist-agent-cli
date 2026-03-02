use crate::db::CacheDb;
use crate::error::CacheError;
use rusqlite::params;
use td_api::models::Section;

impl CacheDb {
    pub fn replace_all_sections(&mut self, sections: &[Section]) -> Result<(), CacheError> {
        let tx = self.conn_mut().transaction()?;
        tx.execute("DELETE FROM sections", [])?;
        for section in sections {
            let raw_json = serde_json::to_string(section)?;
            tx.execute(
                "INSERT INTO sections (id, project_id, name, \"order\", raw_json)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    section.id,
                    section.project_id,
                    section.name,
                    section.order,
                    raw_json
                ],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_all_sections(&self) -> Result<Vec<Section>, CacheError> {
        let mut stmt = self
            .conn()
            .prepare("SELECT raw_json FROM sections ORDER BY \"order\" ASC")?;
        let sections = stmt
            .query_map([], |row| {
                let json: String = row.get(0)?;
                Ok(json)
            })?
            .filter_map(|r| r.ok())
            .filter_map(|json| serde_json::from_str(&json).ok())
            .collect();
        Ok(sections)
    }

    pub fn get_sections_by_project(&self, project_id: &str) -> Result<Vec<Section>, CacheError> {
        let mut stmt = self.conn().prepare(
            "SELECT raw_json FROM sections WHERE project_id = ?1 ORDER BY \"order\" ASC",
        )?;
        let sections = stmt
            .query_map(params![project_id], |row| {
                let json: String = row.get(0)?;
                Ok(json)
            })?
            .filter_map(|r| r.ok())
            .filter_map(|json| serde_json::from_str(&json).ok())
            .collect();
        Ok(sections)
    }

    pub fn find_section_by_name(
        &self,
        name: &str,
        project_id: Option<&str>,
    ) -> Result<Option<Section>, CacheError> {
        let result = if let Some(pid) = project_id {
            self.conn().query_row(
                "SELECT raw_json FROM sections WHERE LOWER(name) = LOWER(?1) AND project_id = ?2",
                params![name, pid],
                |row| row.get::<_, String>(0),
            )
        } else {
            self.conn().query_row(
                "SELECT raw_json FROM sections WHERE LOWER(name) = LOWER(?1)",
                params![name],
                |row| row.get::<_, String>(0),
            )
        };
        match result {
            Ok(json) => Ok(Some(serde_json::from_str(&json)?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(CacheError::Database(e)),
        }
    }

    pub fn upsert_section(&self, section: &Section) -> Result<(), CacheError> {
        let raw_json = serde_json::to_string(section)?;
        self.conn().execute(
            "INSERT OR REPLACE INTO sections (id, project_id, name, \"order\", raw_json)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                section.id,
                section.project_id,
                section.name,
                section.order,
                raw_json
            ],
        )?;
        Ok(())
    }

    pub fn delete_cached_section(&self, id: &str) -> Result<(), CacheError> {
        self.conn()
            .execute("DELETE FROM sections WHERE id = ?1", params![id])?;
        Ok(())
    }
}
