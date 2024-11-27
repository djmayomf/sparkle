// Main library exports and module declarations
mod games;
mod knowledge;
mod moderation;
mod obs;
mod scrapers;
mod security;
mod stream;
mod voice;

// Re-exports for commonly used types
pub use games::launcher::GameLauncher;
pub use knowledge::base::KnowledgeBase;
pub use moderation::content_filter::ContentFilter;
pub use obs::controller::OBSController;
pub use security::defense_system::SecurityDefenseSystem;
pub use stream::scheduler::StreamScheduler;
pub use voice::speech_recognition::SpeechRecognizer;

// Public modules
pub mod games {
    pub use super::games::launcher::GameLauncher;
    pub use super::games::apex::trainer::ApexTrainer;
    pub use super::games::valorant::trainer::ValorantTrainer;
}

pub mod scrapers {
    pub use super::scrapers::anime_scraper::AnimeNewsScraper;
    pub use super::scrapers::casp_scraper::CASPScraper;
    pub use super::scrapers::comptia_scraper::CompTIAScraper;
    pub use super::scrapers::darkreading_scraper::DarkReadingScraper;
    pub use super::scrapers::security_scraper::SecurityNewsScraper;
    pub use super::scrapers::tech_news_scraper::TechNewsScraper;
    pub use super::scrapers::tokusatsu_scraper::TokuScraper;
}

pub mod security {
    pub use super::security::defense_system::SecurityDefenseSystem;
    pub use super::security::vpn_manager::VPNManager;
}

pub mod stream {
    pub use super::stream::scheduler::StreamScheduler;
    pub use super::stream::session_manager::SessionManager;
}

// Dependencies in Cargo.toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
reqwest = { version = "0.11", features = ["json"] }
scraper = "0.17"
pdf = "0.8"
regex = "1.5"
google-cloud-speech = "0.2"
cpal = "0.15"
obs-websocket = "0.5"
fastrand = "2.0"
dotenv = "0.15"

pub mod ai;
pub mod audio;
pub mod config;
pub mod database;
pub mod error;
pub mod events;
pub mod maintenance;
pub mod obs;
pub mod safety;
pub mod scrapers;
pub mod security;
pub mod stream;
pub mod twitch;
pub mod voice;
pub mod youtube;

// Re-export commonly used items
pub use crate::error::{AppError, Result};
pub use crate::database::connection::DatabaseConnection;

// Export public interfaces
pub mod prelude {
    pub use crate::error::{AppError, Result};
    pub use crate::database::connection::DatabaseConnection;
    pub use crate::knowledge::base::KnowledgeBase;
    pub use crate::security::defense_system::SecurityDefenseSystem;
    pub use crate::voice::chat_manager::VoiceChatManager;
}

pub mod utils {
    pub mod base64;
} 