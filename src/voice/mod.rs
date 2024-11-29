pub mod chat_manager;
pub mod speech_recognition;
pub mod emotion_handler;
pub mod personality_filter;
pub mod sync_manager;
pub mod karaoke;
pub mod config;

pub use karaoke::{KaraokeManager, KaraokeSong, KaraokeState};
pub use config::VoiceConfig;
