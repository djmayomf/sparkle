use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("OBS error: {0}")]
    OBS(String),

    #[error("Voice error: {0}")]
    Voice(String),

    #[error("System error: {0}")]
    System(String),
}

pub type Result<T> = std::result::Result<T, AppError>; 