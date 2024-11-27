use rust_bert::pipelines::text_classification::{TextClassificationModel, TextClassificationConfig};
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentFilter {
    allowed_swears: HashSet<String>,      // Mild swears that are allowed
    banned_words: HashSet<String>,        // Completely banned words
    sensitive_topics: HashSet<String>,
    replacement_words: HashMap<String, String>,
    model: TextClassificationModel,
}

impl ContentFilter {
    pub fn new() -> Self {
        Self {
            allowed_swears: Self::init_allowed_swears(),
            banned_words: Self::init_banned_words(),
            sensitive_topics: Self::init_sensitive_topics(),
            replacement_words: Self::init_replacements(),
            model: TextClassificationModel::new(Default::default()).unwrap(),
        }
    }

    fn init_allowed_swears() -> HashSet<String> {
        let mut allowed = HashSet::new();
        // Mild swears that won't get filtered
        allowed.insert("damn".to_string());
        allowed.insert("hell".to_string());
        allowed.insert("crap".to_string());
        allowed.insert("shoot".to_string());
        allowed.insert("dang".to_string());
        allowed
    }

    fn init_banned_words() -> HashSet<String> {
        let mut words = HashSet::new();
        // Severe profanity and slurs stay banned
        words.insert("hate".to_string());
        words.insert("slur".to_string());
        // Add other banned words...
        words
    }

    fn init_sensitive_topics() -> HashSet<String> {
        let mut topics = HashSet::new();
        topics.insert("politics".to_string());
        topics.insert("religion".to_string());
        topics.insert("discrimination".to_string());
        topics.insert("nsfw".to_string());
        topics
    }

    pub fn filter_message(&self, message: &str) -> Option<String> {
        let lower_message = message.to_lowercase();
        
        // Check for banned words first
        if self.contains_banned_words(&lower_message) {
            return None;
        }

        // Check for sensitive topics
        if self.contains_sensitive_topics(&lower_message) {
            return None;
        }

        // Allow message if it only contains allowed swears
        if self.contains_only_allowed_swears(&lower_message) {
            return Some(message.to_string());
        }

        // Apply replacements for other cases
        Some(self.apply_replacements(message))
    }

    fn contains_only_allowed_swears(&self, message: &str) -> bool {
        let words: Vec<&str> = message.split_whitespace().collect();
        words.iter().all(|word| {
            self.allowed_swears.contains(&word.to_string()) || 
            !self.is_profanity(word)
        })
    }

    fn is_profanity(&self, word: &str) -> bool {
        // Check if word is any kind of swear/profanity
        // Returns false for allowed swears
        if self.allowed_swears.contains(word) {
            return false;
        }
        
        // Check against a list of known profanity
        self.banned_words.contains(word)
    }

    fn contains_banned_words(&self, message: &str) -> bool {
        self.banned_words.iter()
            .any(|word| message.contains(word))
    }

    fn contains_sensitive_topics(&self, message: &str) -> bool {
        self.sensitive_topics.iter()
            .any(|topic| message.contains(topic))
    }

    fn apply_replacements(&self, message: &str) -> String {
        let mut filtered = message.to_string();
        for (problematic, safe) in &self.replacement_words {
            filtered = filtered.replace(problematic, safe);
        }
        filtered
    }

    pub fn is_safe_for_stream(&self, message: &str) -> bool {
        // Use ML model for more subtle content checking
        let prediction = self.model.predict(&[message]);
        
        match prediction[0].as_str() {
            "safe" | "mild" => true,
            _ => false,
        }
    }

    pub fn get_safe_response(&self, message: &str) -> Option<String> {
        // First apply content filter
        let filtered = self.filter_message(message)?;
        
        // Then verify it's safe
        if self.is_safe_for_stream(&filtered) {
            Some(filtered)
        } else {
            Some("Let's keep things fun and friendly! ✨".to_string())
        }
    }

    pub fn update_allowed_swears(&mut self, word: String, allow: bool) {
        if allow {
            self.allowed_swears.insert(word);
        } else {
            self.allowed_swears.remove(&word);
        }
    }
}

// Add this to the VoiceChatManager
impl VoiceChatManager {
    async fn process_message(&mut self, message: &str) -> Option<String> {
        // Apply content filter before processing
        self.content_filter.get_safe_response(message)
    }
}

// Add this to the PersonalityFilter
impl PersonalityFilter {
    fn filter_message(&mut self, message: &str, context: &str) -> String {
        // First apply content safety filter
        if let Some(safe_message) = self.content_filter.get_safe_response(message) {
            // Then apply personality
            self.apply_personality(&safe_message, context)
        } else {
            // Fall back to safe default responses
            "Let's keep things positive and fun! ✨".to_string()
        }
    }
} 