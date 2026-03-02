use thiserror::Error;

/// Exit codes for CLI process
pub mod exit_code {
    pub const SUCCESS: i32 = 0;
    pub const GENERAL: i32 = 1;
    pub const AUTH: i32 = 2;
    pub const NETWORK: i32 = 3;
    pub const NOT_FOUND: i32 = 4;
    pub const VALIDATION: i32 = 5;
    pub const CACHE: i32 = 10;
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Authentication failed: {message}")]
    Auth { message: String },

    #[error("Rate limited, retry after {retry_after_secs}s")]
    RateLimited { retry_after_secs: u64 },

    #[error("Resource not found: {resource}")]
    NotFound { resource: String },

    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Deserialization error: {0}")]
    Deserialization(#[from] serde_json::Error),

    #[error("Server error ({status}): {message}")]
    Server { status: u16, message: String },

    #[error("Request timeout")]
    Timeout,
}

impl ApiError {
    pub fn exit_code(&self) -> i32 {
        match self {
            ApiError::Auth { .. } => exit_code::AUTH,
            ApiError::RateLimited { .. } => exit_code::NETWORK,
            ApiError::NotFound { .. } => exit_code::NOT_FOUND,
            ApiError::Validation { .. } => exit_code::VALIDATION,
            ApiError::Network(_) => exit_code::NETWORK,
            ApiError::Timeout => exit_code::NETWORK,
            ApiError::Deserialization(_) | ApiError::Server { .. } => exit_code::GENERAL,
        }
    }
}
