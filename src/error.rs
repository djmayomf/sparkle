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

    #[error("Voice processing error: {0}")]
    Voice(String),

    #[error("Neural chat error: {0}")]
    NeuralChat(String),

    #[error("Knowledge base error: {0}")]
    KnowledgeBase(String),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, AppError>; 