mod config;
mod twitch;
mod nlp;
mod knowledge;
mod events;
mod community;
mod reactions;
mod moderation;
mod ai;
mod memory;
mod stream;
mod autonomy;
mod emotions;
mod emulator;
mod game;
mod safety;
mod maintenance;
mod tts;
mod obs;
mod youtube;
mod security;
mod games;
mod voice;
use games::overwatch::trainer::OverwatchTrainer;
use games::minecraft::trainer::MinecraftTrainer;
use games::input_system::GameInputSystem;
use voice::chat_manager::{VoiceChatManager, VoiceMessage};
use voice::speech_recognition::{SpeechRecognizer, VoiceCommand, CommandType};

use tokio;
use events::subathon::SubathonManager;
use community::manager::CommunityManager;
use reactions::manager::ReactionManager;
use moderation::filter::ContentFilter;
use ai::neural_chat::NeuralChat;
use memory::cache::MemoryCache;
use stream::title_generator::TitleGenerator;
use autonomy::controller::AutonomyController;
use emotions::adapter::EmotionalAdapter;
use emulator::retroarch::RetroArchClient;
use game::input_handler::InputHandler;
use game::state_manager::GameStateManager;
use safety::mod_system::ModSystem;
use maintenance::model_manager::ModelManager;
use maintenance::scheduler::MaintenanceScheduler;
use maintenance::scheduler::TaskType;
use chrono::{DateTime, Utc, Duration};
use std::collections::{HashMap, HashSet};
use obs::controller::OBSController;
use youtube::manager::YouTubeManager;
use security::knowledge_base::SecurityKnowledgeBase;
use stream::interaction_handler::InteractionHandler;
use stream::session_manager::{StreamManager, StreamEvent};
use security::defense_system::{SecurityDefenseSystem, Attack, AttackType};
use crate::database::connection::DatabaseConnection;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use tracing::{info, error, warn};
use tracing_subscriber::{self, EnvFilter};
use crate::error::{AppError, Result};
use crate::database::connection::DatabaseConnection;
use knowledge::auto_updater::{start_knowledge_updater};

// Game engine: Bevy
// use bevy::prelude::*;

// kamen-sparkle_serde
// impl From<u16> for Ipv4Addr {
//     fn from(value: u16) -> Self {
//         Ipv4Addr::new(127, 0, 0, 1)
//     }
// }
//
// /bin
//
// Cargo.toml -> Workspace
//
//

// lib
// pub mod prelude {
// All reexports go here
//
//
// }

// struct State {}
// 
// fn main() -> impl Future<Output = Result<(), Box<dyn std::error::Error>>> {
//      async {
//
//      }
// }
//

// Add this struct to hold application state
pub struct AppState {
    db: DatabaseConnection,
    shutdown: Arc<AtomicBool>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Starting Kamen-Sparkle application");

    // Load environment variables with validation
    dotenv::dotenv().map_err(|e| AppError::Config(e.to_string()))?;
    validate_environment_variables()?;

    // Initialize database with proper error handling and connection pooling
    let db = DatabaseConnection::new()
        .await
        .map_err(|e| {
            error!("Failed to initialize database: {}", e);
            AppError::Database(e)
        })?;

    // Create application state
    let app_state = Arc::new(AppState {
        db: db.clone(),
        shutdown: Arc::new(AtomicBool::new(false)),
    });

    // Set up shutdown handler
    let state_clone = Arc::clone(&app_state);
    ctrlc::set_handler(move || {
        info!("Received shutdown signal");
        state_clone.shutdown.store(true, Ordering::Relaxed);
    })?;

    // Start knowledge auto-updater in background
    let db_clone = db.clone();
    tokio::spawn(async move {
        if let Err(e) = start_knowledge_updater(db_clone.get_pool()).await {
            error!("Knowledge updater error: {}", e);
        }
    });

    // Health check loop
    let health_check_interval = tokio::time::interval(Duration::from_secs(300));
    let db_clone = db.clone();
    tokio::spawn(async move {
        health_check_loop(health_check_interval, db_clone).await;
    });

    // Main application loop with error recovery
    while !app_state.shutdown.load(Ordering::Relaxed) {
        if let Err(e) = run_application_loop(&db).await {
            error!("Application error: {}", e);
            // Add exponential backoff before retry
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }

    info!("Shutting down gracefully");
    Ok(())
}

async fn run_application_loop(db: &DatabaseConnection) -> Result<()> {
    // Initialize core components
    let obs = OBSController::new().await?;
    let voice_chat = VoiceChatManager::new().await?;
    let neural_chat = NeuralChat::new(db.clone()).await?;
    
    // Main loop implementation
    loop {
        tokio::select! {
            // Handle various events
            _ = tokio::time::sleep(Duration::from_millis(100)) => {
                // Regular processing
            }
        }
    }
}

fn validate_environment_variables() -> Result<()> {
    let required_vars = [
        "DATABASE_URL",
        "YOUTUBE_API_KEY",
        "GOOGLE_APPLICATION_CREDENTIALS",
        "OBS_WEBSOCKET_PORT",
        "OBS_WEBSOCKET_PASSWORD",
    ];
    
    for var in required_vars {
        std::env::var(var).map_err(|_| AppError::Config(format!("Missing {}", var)))?;
    }
    Ok(())
}

async fn health_check_loop(mut interval: tokio::time::Interval, db: DatabaseConnection) {
    loop {
        interval.tick().await;
        match check_system_health(&db).await {
            Ok(_) => info!("Health check passed"),
            Err(e) => error!("Health check failed: {}", e),
        }
    }
}

async fn check_system_health(db: &DatabaseConnection) -> Result<()> {
    // Check database connectivity
    db.get_pool().acquire().await.map_err(AppError::Database)?;
    
    // Add other health checks here
    // - Check OBS connection
    // - Check memory usage
    // - Check CPU usage
    // - Check disk space
    
    Ok(())
}
