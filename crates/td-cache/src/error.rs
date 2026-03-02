use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Migration failed: {0}")]
    Migration(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Cache not found: {0}")]
    NotFound(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl CacheError {
    pub fn exit_code(&self) -> i32 {
        td_api::error::exit_code::CACHE
    }
}
