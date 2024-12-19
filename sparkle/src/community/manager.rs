use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommunityMember {
    pub username: String,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub interaction_count: u32,
    pub favorite_topics: Vec<String>,
    pub is_subscriber: bool,
}

pub struct CommunityManager {
    members: HashMap<String, CommunityMember>,
    greeting_cache: Vec<String>,
}

impl CommunityManager {
    pub fn new() -> Self {
        Self {
            members: HashMap::new(),
            greeting_cache: Vec::new(),
        }
    }

    pub fn generate_greeting(&mut self, username: &str) -> String {
        let member = self.members.get(username);
        
        match member {
            Some(m) if m.is_subscriber => {
                format!("Welcome back to the Kawaii Hackers, {}! 💕 It's always great to see our amazing subscribers! (ﾉ◕ヮ◕)ﾉ*:･ﾟ✧", username)
            }
            Some(m) => {
                format!("Welcome back, {}! Ready for more cyber adventures? (◕‿◕✿)", username)
            }
            None => {
                let greeting = format!("Welcome to the Kawaii Hackers, {}! I'm Kamen-Sparkle! (｡♥‿♥｡)", username);
                self.add_new_member(username);
                greeting
            }
        }
    }

    fn add_new_member(&mut self, username: &str) {
        let new_member = CommunityMember {
            username: username.to_string(),
            first_seen: Utc::now(),
            last_seen: Utc::now(),
            interaction_count: 1,
            favorite_topics: Vec::new(),
            is_subscriber: false,
        };
        self.members.insert(username.to_string(), new_member);
    }

    pub fn update_member_activity(&mut self, username: &str) {
        if let Some(member) = self.members.get_mut(username) {
            member.last_seen = Utc::now();
            member.interaction_count += 1;
        }
    }
} 