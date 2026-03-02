use crate::db::CacheDb;
use crate::error::CacheError;
use rusqlite::params;
use td_api::models::Label;

impl CacheDb {
    pub fn replace_all_labels(&mut self, labels: &[Label]) -> Result<(), CacheError> {
        let tx = self.conn_mut().transaction()?;
        tx.execute("DELETE FROM labels", [])?;
        for label in labels {
            let raw_json = serde_json::to_string(label)?;
            tx.execute(
                "INSERT INTO labels (id, name, color, \"order\", is_favorite, raw_json)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    label.id,
                    label.name,
                    label.color,
                    label.order,
                    label.is_favorite,
                    raw_json
                ],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_all_labels(&self) -> Result<Vec<Label>, CacheError> {
        let mut stmt = self
            .conn()
            .prepare("SELECT raw_json FROM labels ORDER BY \"order\" ASC")?;
        let labels = stmt
            .query_map([], |row| {
                let json: String = row.get(0)?;
                Ok(json)
            })?
            .filter_map(|r| r.ok())
            .filter_map(|json| serde_json::from_str(&json).ok())
            .collect();
        Ok(labels)
    }

    pub fn find_label_by_name(&self, name: &str) -> Result<Option<Label>, CacheError> {
        let result: Result<String, _> = self.conn().query_row(
            "SELECT raw_json FROM labels WHERE LOWER(name) = LOWER(?1)",
            params![name],
            |row| row.get(0),
        );
        match result {
            Ok(json) => Ok(Some(serde_json::from_str(&json)?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(CacheError::Database(e)),
        }
    }

    pub fn upsert_label(&self, label: &Label) -> Result<(), CacheError> {
        let raw_json = serde_json::to_string(label)?;
        self.conn().execute(
            "INSERT OR REPLACE INTO labels (id, name, color, \"order\", is_favorite, raw_json)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                label.id,
                label.name,
                label.color,
                label.order,
                label.is_favorite,
                raw_json
            ],
        )?;
        Ok(())
    }

    pub fn delete_cached_label(&self, id: &str) -> Result<(), CacheError> {
        self.conn()
            .execute("DELETE FROM labels WHERE id = ?1", params![id])?;
        Ok(())
    }
}
