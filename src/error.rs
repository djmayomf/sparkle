use std::fmt;
use crate::security::SecurityError;

#[derive(Debug)]
pub enum AppError {
    Security(SecurityError),
    Database(String),
    Network(String),
    Task(String),
    IO(std::io::Error),
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Security(e) => write!(f, "Security error: {:?}", e),
            AppError::Database(e) => write!(f, "Database error: {}", e),
            AppError::Network(e) => write!(f, "Network error: {}", e),
            AppError::Task(e) => write!(f, "Task error: {}", e),
            AppError::IO(e) => write!(f, "IO error: {}", e),
            AppError::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl From<SecurityError> for AppError {
    fn from(err: SecurityError) -> Self {
        AppError::Security(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IO(err)
    }
}

pub type Result<T> = std::result::Result<T, AppError>; 

#[derive(Debug, thiserror::Error)]
pub enum SystemError {
    #[error("Personality core error: {0}")]
    PersonalityError(String),
    
    #[error("Content creation error: {0}")]
    ContentError(String),
    
    #[error("Stream orchestration error: {0}")]
    OrchestrationError(String),
    
    #[error("VRChat integration error: {0}")]
    VRChatError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("System synchronization error: {0}")]
    SyncError(String),
}

pub type Result<T> = std::result::Result<T, SystemError>; 