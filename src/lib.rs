// Remove duplicate module declarations and keep only the public ones
pub mod ai;
pub mod audio;
pub mod autonomy;
pub mod config;
pub mod database;
pub mod error;
pub mod events;
pub mod games {
    pub mod launcher;
    pub mod apex {
        pub mod trainer;
    }
    pub mod valorant {
        pub mod trainer;
    }
}
pub mod knowledge {
    pub mod base;
}
pub mod maintenance;
pub mod moderation {
    pub mod content_filter;
}
pub mod obs {
    pub mod controller;
}
pub mod safety;
pub mod scrapers {
    pub mod anime_scraper;
    pub mod casp_scraper;
    pub mod comptia_scraper;
    pub mod darkreading_scraper;
    pub mod security_scraper;
    pub mod tech_news_scraper;
    pub mod tokusatsu_scraper;
}
pub mod security {
    pub mod defense_system;
    pub mod vpn_manager;

    pub use self::defense_system::{
        SecurityDefenseSystem,
        Permission,
        SecurityError,
    };
}
pub mod stream {
    pub mod scheduler;
    pub mod session_manager;
}
pub mod twitch;
pub mod utils;
pub mod voice {
    pub mod chat_manager;
    pub mod speech_recognition;
}
pub mod youtube;
pub mod automation;
pub mod emotions {
    pub mod adapter;
}

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
    pub use crate::automation::{
        TaskManager,
        AutomatedTask,
        TaskType,
        TaskPriority,
        TaskSchedule,
        TaskStatus,
    };
} 