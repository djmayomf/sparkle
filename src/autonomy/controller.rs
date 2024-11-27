use std::collections::{HashMap, VecDeque};
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InteractionPattern {
    pub pattern_type: String,
    pub occurrences: Vec<DateTime<Utc>>,
    pub frequency: f32,
}

pub struct AutonomyController {
    response_history: VecDeque<String>,
    pattern_tracker: HashMap<String, InteractionPattern>,
    last_response: DateTime<Utc>,
    min_response_interval: Duration,
}

impl AutonomyController {
    pub fn new() -> Self {
        Self {
            response_history: VecDeque::with_capacity(20),
            pattern_tracker: HashMap::new(),
            last_response: Utc::now(),
            min_response_interval: Duration::seconds(2),
        }
    }

    pub fn can_respond(&self) -> bool {
        Utc::now() - self.last_response >= self.min_response_interval
    }

    pub fn check_response(&mut self, response: &str) -> bool {
        // Check for repetition
        if self.is_repetitive(response) {
            return false;
        }

        // Add to history
        self.add_to_history(response);
        true
    }

    fn is_repetitive(&self, response: &str) -> bool {
        // Check last 3 responses for similarity
        let recent_responses: Vec<&String> = self.response_history.iter().take(3).collect();
        recent_responses.iter().any(|&prev| self.similarity_score(prev, response) > 0.8)
    }

    fn similarity_score(&self, a: &str, b: &str) -> f32 {
        // Simple similarity check - can be enhanced with more sophisticated algorithms
        if a == b {
            return 1.0;
        }
        
        let a_words: Vec<&str> = a.split_whitespace().collect();
        let b_words: Vec<&str> = b.split_whitespace().collect();
        
        let common_words = a_words.iter()
            .filter(|word| b_words.contains(word))
            .count();
            
        2.0 * common_words as f32 / (a_words.len() + b_words.len()) as f32
    }

    fn add_to_history(&mut self, response: &str) {
        if self.response_history.len() >= 20 {
            self.response_history.pop_front();
        }
        self.response_history.push_back(response.to_string());
    }

    pub fn track_pattern(&mut self, pattern_type: &str) {
        let now = Utc::now();
        
        self.pattern_tracker
            .entry(pattern_type.to_string())
            .and_modify(|pattern| {
                pattern.occurrences.push(now);
                pattern.frequency = Self::calculate_frequency(&pattern.occurrences);
            })
            .or_insert(InteractionPattern {
                pattern_type: pattern_type.to_string(),
                occurrences: vec![now],
                frequency: 0.0,
            });
    }

    fn calculate_frequency(occurrences: &[DateTime<Utc>]) -> f32 {
        if occurrences.len() < 2 {
            return 0.0;
        }
        
        let duration = occurrences.last().unwrap() - occurrences[0];
        occurrences.len() as f32 / duration.num_minutes() as f32
    }
} 