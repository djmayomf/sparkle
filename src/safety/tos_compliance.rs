use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use fastrand;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TOSViolation {
    pub violation_type: ViolationType,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ViolationType {
    SelfPromotion,
    PersonalInfo,
    Harassment,
    NSFW,
    Spoilers,
    UnsolicitedAdvice,
    BackseatingGaming,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Severity {
    Low,    // Just needs a gentle reminder
    Medium, // Requires clear warning
    High,   // Needs mod attention
}

pub struct TOSCompliance {
    tos_triggers: HashSet<String>,
    friendly_responses: HashMap<ViolationType, Vec<String>>,
    last_warning: Option<DateTime<Utc>>,
}

impl TOSCompliance {
    pub fn new() -> Self {
        Self {
            tos_triggers: Self::init_triggers(),
            friendly_responses: Self::init_responses(),
            last_warning: None,
        }
    }

    fn init_triggers() -> HashSet<String> {
        let mut triggers = HashSet::new();
        // Add common TOS-violating requests
        triggers.insert("show face".to_string());
        triggers.insert("real age".to_string());
        triggers.insert("real name".to_string());
        triggers.insert("where live".to_string());
        triggers.insert("personal info".to_string());
        triggers.insert("check my channel".to_string());
        triggers.insert("watch my stream".to_string());
        triggers.insert("follow me".to_string());
        triggers
    }

    fn init_responses() -> HashMap<ViolationType, Vec<String>> {
        let mut responses = HashMap::new();
        
        responses.insert(ViolationType::SelfPromotion, vec![
            "Aww, sorry bestie! Self-promotion isn't allowed here! Let's keep the chat fun for everyone! (｡•́︿•̀｡)".to_string(),
            "Oopsie! We can't do self-promo here! It's against TOS! Let's chat about something else! ✨".to_string(),
        ]);

        responses.insert(ViolationType::PersonalInfo, vec![
            "Hehe, nice try! But I have to keep some mysteries! It's for everyone's safety! (◕‿◕✿)".to_string(),
            "Sorry! Can't share personal info - Twitch rules! Let's talk about games instead! 🎮".to_string(),
        ]);

        responses.insert(ViolationType::BackseatingGaming, vec![
            "Let me figure this out myself! No backseat gaming please! (｡♥‿♥｡)".to_string(),
            "I appreciate the help, but let's avoid backseat gaming! Part of the fun is learning! ✨".to_string(),
        ]);

        responses.insert(ViolationType::NSFW, vec![
            "Yikes! Let's keep it family friendly here! (｡•́︿•̀｡)".to_string(),
            "This is a kawaii stream! We keep things PG! 🌸".to_string(),
        ]);

        responses
    }

    pub async fn check_message(&mut self, message: &str) -> Option<String> {
        let lower_message = message.to_lowercase();
        
        // Check for TOS-violating requests
        if let Some(violation_type) = self.detect_violation(&lower_message) {
            return Some(self.get_friendly_response(violation_type));
        }
        None
    }

    fn detect_violation(&self, message: &str) -> Option<ViolationType> {
        if message.contains("follow") && (message.contains("my") || message.contains("me")) {
            return Some(ViolationType::SelfPromotion);
        }
        
        if message.contains("face") || message.contains("real name") || message.contains("where") {
            return Some(ViolationType::PersonalInfo);
        }

        if message.contains("you should") && (message.contains("play") || message.contains("do")) {
            return Some(ViolationType::BackseatingGaming);
        }

        None
    }

    fn get_friendly_response(&self, violation_type: ViolationType) -> String {
        if let Some(responses) = self.friendly_responses.get(&violation_type) {
            responses[fastrand::usize(..responses.len())].clone()
        } else {
            "Sorry! Can't do that - it's against TOS! Let's keep things fun and safe! ✨".to_string()
        }
    }
}

// Add this to the VoiceChatManager
impl VoiceChatManager {
    async fn check_voice_content(&mut self, text: &str) -> Result<(), TOSViolation> {
        self.tos_compliance.check_message(text).await?;
        Ok(())
    }
}

// Add this to the StreamManager
impl StreamManager {
    async fn ensure_tos_compliance(&mut self) -> Result<(), TOSViolation> {
        // Regular checks during stream
        self.tos_compliance.check_stream_content("music", &self.current_music).await?;
        self.tos_compliance.check_stream_content("gameplay", &self.current_game).await?;
        Ok(())
    }
} 