use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tokio::sync::broadcast;
use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModAction {
    pub action_type: ModActionType,
    pub user: String,
    pub reason: String,
    pub duration: Option<Duration>,
    pub timestamp: DateTime<Utc>,
    pub mod_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModActionType {
    Delete,
    Timeout,
    Ban,
    Warning,
    ModCall,
}

#[derive(Debug)]
pub struct ModSystem {
    moderators: HashSet<String>,
    action_history: Vec<ModAction>,
    warning_counts: HashMap<String, u32>,
    banned_users: HashSet<String>,
    event_sender: broadcast::Sender<ModAction>,
}

impl ModSystem {
    pub fn new() -> (Self, broadcast::Receiver<ModAction>) {
        let (tx, rx) = broadcast::channel(100);
        
        (Self {
            moderators: HashSet::new(),
            action_history: Vec::new(),
            warning_counts: HashMap::new(),
            banned_users: HashSet::new(),
            event_sender: tx,
        }, rx)
    }

    pub fn add_moderator(&mut self, username: String) {
        self.moderators.insert(username);
    }

    pub async fn handle_violation(&mut self, user: &str, violation_type: &str, severity: u8) -> Result<(), String> {
        let action = match (severity, self.warning_counts.get(user).unwrap_or(&0)) {
            (s, _) if s >= 8 => ModActionType::Ban,
            (s, w) if s >= 5 || *w >= 3 => ModActionType::Timeout,
            _ => ModActionType::Warning,
        };

        self.execute_mod_action(action, user.to_string(), violation_type.to_string()).await
    }

    pub async fn call_mods(&mut self, reason: String) -> Result<(), String> {
        let action = ModAction {
            action_type: ModActionType::ModCall,
            user: "System".to_string(),
            reason,
            duration: None,
            timestamp: Utc::now(),
            mod_name: "Kamen-Sparkle".to_string(),
        };

        self.broadcast_action(action).await;
        Ok(())
    }

    async fn execute_mod_action(&mut self, action_type: ModActionType, user: String, reason: String) -> Result<(), String> {
        let duration = match action_type {
            ModActionType::Timeout => Some(Duration::minutes(10)),
            _ => None,
        };

        let action = ModAction {
            action_type,
            user: user.clone(),
            reason,
            duration,
            timestamp: Utc::now(),
            mod_name: "Kamen-Sparkle".to_string(),
        };

        match action.action_type {
            ModActionType::Warning => {
                *self.warning_counts.entry(user).or_insert(0) += 1;
            }
            ModActionType::Ban => {
                self.banned_users.insert(user);
            }
            _ => {}
        }

        self.action_history.push(action.clone());
        self.broadcast_action(action).await;
        Ok(())
    }

    async fn broadcast_action(&self, action: ModAction) {
        let _ = self.event_sender.send(action);
    }

    pub fn is_banned(&self, user: &str) -> bool {
        self.banned_users.contains(user)
    }

    pub fn is_moderator(&self, user: &str) -> bool {
        self.moderators.contains(user)
    }

    pub fn get_warning_count(&self, user: &str) -> u32 {
        *self.warning_counts.get(user).unwrap_or(&0)
    }
} 