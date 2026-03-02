use crate::db::CacheDb;
use crate::error::CacheError;
use tracing::info;

const MIGRATIONS: &[(&str, &str)] =
    &[("v001_initial", include_str!("migrations/v001_initial.sql"))];

pub fn run(db: &mut CacheDb) -> Result<(), CacheError> {
    db.conn().execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER NOT NULL,
            name TEXT NOT NULL,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
    )?;

    let current_version: i32 = db.conn().query_row(
        "SELECT COALESCE(MAX(version), 0) FROM schema_version",
        [],
        |row| row.get(0),
    )?;

    for (i, (name, sql)) in MIGRATIONS.iter().enumerate() {
        let version = (i + 1) as i32;
        if version > current_version {
            info!(version, name, "Applying migration");
            db.conn()
                .execute_batch(sql)
                .map_err(|e| CacheError::Migration(format!("{name}: {e}")))?;
            db.conn().execute(
                "INSERT INTO schema_version (version, name) VALUES (?1, ?2)",
                rusqlite::params![version, name],
            )?;
        }
    }

    Ok(())
}
