use crate::db::CacheDb;
use crate::error::CacheError;
use rusqlite::params;
use td_api::models::Project;

impl CacheDb {
    pub fn upsert_project(&self, project: &Project) -> Result<(), CacheError> {
        let raw_json = serde_json::to_string(project)?;
        self.conn().execute(
            "INSERT OR REPLACE INTO projects (
                id, name, color, parent_id, \"order\", is_favorite,
                is_inbox_project, is_team_inbox, view_style, url, raw_json
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                project.id,
                project.name,
                project.color,
                project.parent_id,
                project.order,
                project.is_favorite,
                project.is_inbox_project,
                project.is_team_inbox,
                project.view_style,
                project.url,
                raw_json,
            ],
        )?;
        Ok(())
    }

    pub fn replace_all_projects(&mut self, projects: &[Project]) -> Result<(), CacheError> {
        let tx = self.conn_mut().transaction()?;
        tx.execute("DELETE FROM projects", [])?;
        for project in projects {
            let raw_json = serde_json::to_string(project)?;
            tx.execute(
                "INSERT INTO projects (
                    id, name, color, parent_id, \"order\", is_favorite,
                    is_inbox_project, is_team_inbox, view_style, url, raw_json
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![
                    project.id,
                    project.name,
                    project.color,
                    project.parent_id,
                    project.order,
                    project.is_favorite,
                    project.is_inbox_project,
                    project.is_team_inbox,
                    project.view_style,
                    project.url,
                    raw_json,
                ],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_all_projects(&self) -> Result<Vec<Project>, CacheError> {
        let mut stmt = self
            .conn()
            .prepare("SELECT raw_json FROM projects ORDER BY \"order\" ASC")?;
        let projects = stmt
            .query_map([], |row| {
                let json: String = row.get(0)?;
                Ok(json)
            })?
            .filter_map(|r| r.ok())
            .filter_map(|json| serde_json::from_str(&json).ok())
            .collect();
        Ok(projects)
    }

    pub fn get_project(&self, id: &str) -> Result<Project, CacheError> {
        let json: String = self
            .conn()
            .query_row(
                "SELECT raw_json FROM projects WHERE id = ?1",
                params![id],
                |row| row.get(0),
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    CacheError::NotFound(format!("project {id}"))
                }
                other => CacheError::Database(other),
            })?;
        Ok(serde_json::from_str(&json)?)
    }

    pub fn find_project_by_name(&self, name: &str) -> Result<Option<Project>, CacheError> {
        let result: Result<String, _> = self.conn().query_row(
            "SELECT raw_json FROM projects WHERE LOWER(name) = LOWER(?1)",
            params![name],
            |row| row.get(0),
        );
        match result {
            Ok(json) => Ok(Some(serde_json::from_str(&json)?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(CacheError::Database(e)),
        }
    }

    pub fn get_inbox_project(&self) -> Result<Option<Project>, CacheError> {
        let result: Result<String, _> = self.conn().query_row(
            "SELECT raw_json FROM projects WHERE is_inbox_project = 1",
            [],
            |row| row.get(0),
        );
        match result {
            Ok(json) => Ok(Some(serde_json::from_str(&json)?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(CacheError::Database(e)),
        }
    }

    pub fn delete_cached_project(&self, id: &str) -> Result<(), CacheError> {
        self.conn()
            .execute("DELETE FROM projects WHERE id = ?1", params![id])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_project(id: &str, name: &str) -> Project {
        Project {
            id: id.to_string(),
            name: name.to_string(),
            color: Some("blue".to_string()),
            parent_id: None,
            order: Some(1),
            is_favorite: false,
            is_inbox_project: false,
            is_team_inbox: false,
            view_style: Some("list".to_string()),
            url: None,
        }
    }

    #[test]
    fn test_upsert_and_get_project() {
        let db = CacheDb::open_in_memory().unwrap();
        db.upsert_project(&sample_project("p1", "Work")).unwrap();
        let proj = db.get_project("p1").unwrap();
        assert_eq!(proj.name, "Work");
    }

    #[test]
    fn test_find_project_by_name() {
        let db = CacheDb::open_in_memory().unwrap();
        db.upsert_project(&sample_project("p1", "Work")).unwrap();

        let found = db.find_project_by_name("work").unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "p1");

        let not_found = db.find_project_by_name("missing").unwrap();
        assert!(not_found.is_none());
    }

    #[test]
    fn test_replace_all_projects() {
        let mut db = CacheDb::open_in_memory().unwrap();
        db.upsert_project(&sample_project("p1", "Old")).unwrap();
        db.replace_all_projects(&[sample_project("p2", "New")])
            .unwrap();

        let all = db.get_all_projects().unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].id, "p2");
    }
}
