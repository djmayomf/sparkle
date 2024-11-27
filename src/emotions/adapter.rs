use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use chrono::{DateTime, Utc, Duration};
use fastrand;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmotionalState {
    pub mood: Mood,
    pub energy: f32,
    pub excitement: f32,
    pub last_change: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mood {
    Happy,
    Excited,
    Focused,
    Tired,
    Playful,
}

#[derive(Debug)]
pub struct EmotionalAdapter {
    current_state: EmotionalState,
    chat_history: VecDeque<(DateTime<Utc>, f32)>, // timestamp and sentiment
    max_history: usize,
}

impl EmotionalAdapter {
    pub fn new() -> Self {
        Self {
            current_state: EmotionalState {
                mood: Mood::Happy,
                energy: 0.8,
                excitement: 0.5,
                last_change: Utc::now(),
            },
            chat_history: VecDeque::with_capacity(100),
            max_history: 100,
        }
    }

    pub fn adapt_response(&mut self, base_response: &str, chat_sentiment: f32) -> String {
        self.update_emotional_state(chat_sentiment);
        
        let emoji = self.select_emoji();
        let tone = self.adjust_tone(base_response);
        
        format!("{} {}", tone, emoji)
    }

    pub fn update_emotional_state(&mut self, chat_sentiment: f32) {
        let now = Utc::now();
        
        // Natural mood transitions
        if (now - self.current_state.last_change) > Duration::minutes(30) {
            self.current_state.energy = (self.current_state.energy * 0.8) + (fastrand::f32() * 0.2);
            
            // Random mood changes
            if fastrand::f32() < 0.3 {
                self.current_state.mood = match fastrand::u8(0..5) {
                    0 => Mood::Happy,
                    1 => Mood::Excited,
                    2 => Mood::Focused,
                    3 => Mood::Tired,
                    _ => Mood::Playful,
                };
            }
        }

        // Event-based mood changes
        match chat_sentiment {
            sentiment if sentiment > 0.7 => {
                self.current_state.excitement += 0.3;
                self.current_state.mood = Mood::Happy;
            }
            sentiment if sentiment < 0.3 => {
                self.current_state.excitement -= 0.1;
                self.current_state.energy -= 0.1;
            }
            _ => {}
        }

        self.current_state.last_change = now;
    }

    fn select_emoji(&self) -> &str {
        match (self.current_state.mood, self.current_state.energy) {
            (mood, energy) if mood > 0.7 && energy > 0.7 => "(ﾉ◕ヮ◕)ﾉ*:･ﾟ✧",
            (mood, _) if mood > 0.7 => "(◕‿◕✿)",
            (mood, _) if mood < 0.3 => "(｡•́︿•̀｡)",
            _ => "(｡◕‿◕｡)",
        }
    }

    fn adjust_tone(&self, response: &str) -> String {
        if self.current_state.energy > 0.8 {
            format!("{}!", response)
        } else if self.current_state.base_mood < 0.3 {
            format!("*softly* {}", response)
        } else {
            response.to_string()
        }
    }

    fn calculate_energy_level(&self) -> f32 {
        // Energy decreases over stream duration but can be boosted by positive interactions
        let base_energy = 0.8;
        let stream_fatigue = self.chat_history.len() as f32 / self.max_history as f32;
        (base_energy - 0.3 * stream_fatigue).max(0.2)
    }

    fn calculate_engagement(&self) -> f32 {
        // Engagement based on recent chat activity and sentiment variation
        let recent_messages = self.chat_history.len().min(10) as f32 / 10.0;
        let sentiment_variance = self.calculate_sentiment_variance();
        (recent_messages + sentiment_variance) / 2.0
    }

    fn calculate_sentiment_variance(&self) -> f32 {
        if self.chat_history.len() < 2 {
            return 0.5;
        }
        
        let mean = self.chat_history.iter()
            .map(|(_, sentiment)| sentiment)
            .sum::<f32>() / self.chat_history.len() as f32;
            
        let variance = self.chat_history.iter()
            .map(|(_, &sentiment)| (sentiment - mean).powi(2))
            .sum::<f32>() / self.chat_history.len() as f32;
            
        variance.sqrt().min(1.0)
    }
} 