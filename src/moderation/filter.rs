use rust_bert::pipelines::text_classification::{TextClassificationModel, TextClassificationConfig};
use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModerationType {
    pub severity: u8,  // 0-10 scale
    pub category: String,
    pub action: ModAction,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ModAction {
    Delete,
    Timeout(u32),  // seconds
    Ban,
    Alert,
}

pub struct ContentFilter {
    model: TextClassificationModel,
    banned_phrases: HashSet<String>,
    personal_info_patterns: Vec<regex::Regex>,
}

impl ContentFilter {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = TextClassificationConfig::default();
        let model = TextClassificationModel::new(config)?;

        let personal_info_patterns = vec![
            regex::Regex::new(r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b")?, // Phone numbers
            regex::Regex::new(r"\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}\b")?, // Emails
            regex::Regex::new(r"\b\d{3}[-]?\d{2}[-]?\d{4}\b")?, // SSN
        ];

        Ok(Self {
            model,
            banned_phrases: Self::init_banned_phrases(),
            personal_info_patterns,
        })
    }

    fn init_banned_phrases() -> HashSet<String> {
        let mut phrases = HashSet::new();
        // Add banned phrases, slurs, and harmful content patterns
        phrases.insert("slur1".to_string());
        phrases.insert("slur2".to_string());
        // Add more as needed
        phrases
    }

    pub async fn analyze_message(&self, message: &str) -> Option<ModerationType> {
        // Check for banned phrases
        if self.contains_banned_phrase(message) {
            return Some(ModerationType {
                severity: 10,
                category: "hate_speech".to_string(),
                action: ModAction::Ban,
            });
        }

        // Check for personal information
        if self.contains_personal_info(message) {
            return Some(ModerationType {
                severity: 8,
                category: "doxxing".to_string(),
                action: ModAction::Delete,
            });
        }

        // Use ML model for content classification
        let prediction = self.model.predict(&[message]);
        self.evaluate_prediction(&prediction[0])
    }

    fn contains_banned_phrase(&self, message: &str) -> bool {
        let lower_message = message.to_lowercase();
        self.banned_phrases.iter().any(|phrase| lower_message.contains(phrase))
    }

    fn contains_personal_info(&self, message: &str) -> bool {
        self.personal_info_patterns.iter().any(|pattern| pattern.is_match(message))
    }

    fn evaluate_prediction(&self, prediction: &str) -> Option<ModerationType> {
        match prediction {
            p if p.contains("hate") => Some(ModerationType {
                severity: 9,
                category: "hate_speech".to_string(),
                action: ModAction::Ban,
            }),
            p if p.contains("political") => Some(ModerationType {
                severity: 5,
                category: "political".to_string(),
                action: ModAction::Delete,
            }),
            _ => None,
        }
    }
} 