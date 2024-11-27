use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::time::{Duration, Instant};

#[derive(Debug, Serialize, Deserialize)]
pub struct Reaction {
    pub trigger: String,
    pub audio_file: Option<String>,
    pub animation_name: Option<String>,
    pub cooldown: Duration,
}

pub struct ReactionManager {
    reactions: HashMap<String, Reaction>,
    last_used: HashMap<String, Instant>,
    engagement_phrases: Vec<String>,
    last_engagement: Instant,
}

impl ReactionManager {
    pub fn new() -> Self {
        Self {
            reactions: Self::init_reactions(),
            last_used: HashMap::new(),
            engagement_phrases: Self::init_engagement_phrases(),
            last_engagement: Instant::now(),
        }
    }

    fn init_reactions() -> HashMap<String, Reaction> {
        let mut reactions = HashMap::new();
        
        reactions.insert(
            "headpat".to_string(),
            Reaction {
                trigger: "!headpat".to_string(),
                audio_file: Some("sounds/happy_giggle.wav".to_string()),
                animation_name: Some("blush".to_string()),
                cooldown: Duration::from_secs(30),
            }
        );

        // Add more reactions...
        reactions
    }

    fn init_engagement_phrases() -> Vec<String> {
        vec![
            "Remember to stay hydrated, Kawaii Hackers! 💦".to_string(),
            "You're all amazing! Thanks for hanging out! ✨".to_string(),
            "Don't forget to stretch! Healthy hackers are happy hackers! 🌟".to_string(),
        ]
    }

    pub async fn try_reaction(&mut self, trigger: &str) -> Option<&Reaction> {
        if let Some(reaction) = self.reactions.get(trigger) {
            let now = Instant::now();
            if let Some(last_time) = self.last_used.get(trigger) {
                if now.duration_since(*last_time) < reaction.cooldown {
                    return None;
                }
            }
            self.last_used.insert(trigger.to_string(), now);
            Some(reaction)
        } else {
            None
        }
    }

    pub fn get_random_engagement(&mut self) -> Option<&String> {
        let now = Instant::now();
        if now.duration_since(self.last_engagement) < Duration::from_secs(300) {
            return None;
        }
        
        self.last_engagement = now;
        Some(&self.engagement_phrases[fastrand::usize(..self.engagement_phrases.len())])
    }
} 