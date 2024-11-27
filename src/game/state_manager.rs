use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::broadcast;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub event_type: GameEventType,
    pub data: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEventType {
    LevelUp,
    Achievement,
    Death,
    HighScore,
    BossFight,
}

pub struct GameStateManager {
    current_score: u32,
    high_scores: HashMap<String, u32>,
    event_sender: broadcast::Sender<GameEvent>,
    achievements: Vec<String>,
}

impl GameStateManager {
    pub fn new() -> (Self, broadcast::Receiver<GameEvent>) {
        let (tx, rx) = broadcast::channel(100);
        
        (Self {
            current_score: 0,
            high_scores: HashMap::new(),
            event_sender: tx,
            achievements: Vec::new(),
        }, rx)
    }

    pub async fn update_score(&mut self, new_score: u32) {
        self.current_score = new_score;
        
        // Check for high score
        if self.is_high_score(new_score) {
            self.broadcast_event(GameEventType::HighScore, 
                format!("New high score: {}! ✨", new_score)).await;
        }
    }

    pub async fn handle_death(&mut self) {
        self.broadcast_event(GameEventType::Death,
            "Oops! Let's try again! (｡•́︿•̀｡)".to_string()).await;
            
        // Reset current score
        self.current_score = 0;
    }

    pub async fn unlock_achievement(&mut self, achievement: String) {
        if !self.achievements.contains(&achievement) {
            self.achievements.push(achievement.clone());
            self.broadcast_event(GameEventType::Achievement,
                format!("Achievement unlocked: {}! 🏆", achievement)).await;
        }
    }

    async fn broadcast_event(&self, event_type: GameEventType, data: String) {
        let event = GameEvent {
            event_type,
            data,
            timestamp: chrono::Utc::now(),
        };
        
        // Ignore send errors (no active receivers)
        let _ = self.event_sender.send(event);
    }

    fn is_high_score(&self, score: u32) -> bool {
        self.high_scores.values().all(|&high_score| score > high_score)
    }

    pub fn get_current_score(&self) -> u32 {
        self.current_score
    }

    pub fn get_achievements(&self) -> &Vec<String> {
        &self.achievements
    }
} 