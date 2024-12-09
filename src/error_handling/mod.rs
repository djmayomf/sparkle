use std::error::Error;
use std::fmt;
use tokio::sync::broadcast;

#[derive(Debug)]
pub enum SystemError {
    Resource(String),
    Performance(String),
    Network(String),
    Database(String),
}

impl Error for SystemError {}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SystemError::Resource(msg) => write!(f, "Resource error: {}", msg),
            SystemError::Performance(msg) => write!(f, "Performance error: {}", msg),
            SystemError::Network(msg) => write!(f, "Network error: {}", msg),
            SystemError::Database(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

pub struct ErrorHandler {
    error_tx: broadcast::Sender<SystemError>,
    recovery_strategies: HashMap<String, Box<dyn RecoveryStrategy>>,
}

impl ErrorHandler {
    pub fn new() -> (Self, broadcast::Receiver<SystemError>) {
        let (tx, rx) = broadcast::channel(100);
        (Self {
            error_tx: tx,
            recovery_strategies: HashMap::new(),
        }, rx)
    }

    pub async fn handle_error(&self, error: SystemError) {
        // Log error
        tracing::error!("System error: {:?}", error);
        
        // Attempt recovery
        if let Some(strategy) = self.get_recovery_strategy(&error) {
            if let Err(e) = strategy.attempt_recovery(&error).await {
                tracing::error!("Recovery failed: {:?}", e);
            }
        }
        
        // Broadcast error
        if let Err(e) = self.error_tx.send(error) {
            tracing::error!("Error broadcasting failed: {:?}", e);
        }
    }
} 