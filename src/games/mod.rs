pub mod launcher;
pub mod marvel_rivals;
pub mod poe2;
pub mod apex;
pub mod fortnite;
pub mod league;
pub mod minecraft;
pub mod mtga;
pub mod overwatch;
pub mod valorant;
pub mod yugioh;

// Re-export commonly used types
pub use marvel_rivals::trainer::MarvelRivalsTrainer;
pub use poe2::trainer::POE2Trainer; 