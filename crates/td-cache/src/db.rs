use crate::error::CacheError;
use crate::migrations;
use rusqlite::Connection;
use std::path::Path;
use tracing::info;

pub struct CacheDb {
    conn: Connection,
}

impl CacheDb {
    pub fn open(path: &Path) -> Result<Self, CacheError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        let mut db = Self { conn };
        migrations::run(&mut db)?;
        info!("Cache database opened at {}", path.display());
        Ok(db)
    }

    pub fn open_in_memory() -> Result<Self, CacheError> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch("PRAGMA foreign_keys=ON;")?;
        let mut db = Self { conn };
        migrations::run(&mut db)?;
        Ok(db)
    }

    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    pub fn conn_mut(&mut self) -> &mut Connection {
        &mut self.conn
    }
}
