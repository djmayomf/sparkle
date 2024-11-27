use kamen_sparkle::prelude::*;
use tracing::{info, error};
use tokio::time::{self, Duration};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging with better formatting
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    // Load environment variables
    dotenv::dotenv().map_err(|e| AppError::Config(e.to_string()))?;
    validate_environment_variables()?;

    // Initialize database connection with retry logic
    let db = retry_with_backoff(|| DatabaseConnection::new()).await?;
    
    // Initialize voice chat with retry logic
    let voice = retry_with_backoff(|| VoiceChatManager::new()).await?;

    info!("Kamen Sparkle initialized successfully");
    
    // Set up graceful shutdown
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    
    ctrlc::set_handler(move || {
        info!("Shutdown signal received");
        r.store(false, Ordering::SeqCst);
    }).map_err(|e| AppError::System(e.to_string()))?;

    // Main application loop with error recovery
    while running.load(Ordering::SeqCst) {
        if let Err(e) = run_application_loop(&db, &voice).await {
            error!("Application error: {}", e);
            time::sleep(Duration::from_secs(1)).await;
        }
    }

    info!("Shutting down gracefully");
    Ok(())
}

async fn run_application_loop(_db: &DatabaseConnection, _voice: &VoiceChatManager) -> Result<()> {
    // Process events in batches for better performance
    let mut interval = time::interval(Duration::from_millis(100));
    
    loop {
        interval.tick().await;
        
        // Add your main processing logic here
        // For example:
        // - Process voice commands
        // - Update knowledge base
        // - Handle stream events
    }
}

async fn retry_with_backoff<F, Fut, T>(f: F) -> Result<T> 
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut retries = 0;
    let max_retries = 3;

    loop {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                retries += 1;
                if retries >= max_retries {
                    return Err(e);
                }
                let delay = Duration::from_secs(2u64.pow(retries));
                error!("Retry attempt {}/{} after error: {}. Waiting {:?}", 
                    retries, max_retries, e, delay);
                time::sleep(delay).await;
            }
        }
    }
}

fn validate_environment_variables() -> Result<()> {
    let required_vars = [
        "DATABASE_URL",
        "GOOGLE_API_KEY",
    ];
    
    for var in required_vars {
        std::env::var(var).map_err(|_| AppError::Config(format!("Missing {}", var)))?;
    }
    Ok(())
} 