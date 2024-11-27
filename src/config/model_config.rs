use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelConfig {
    pub model_id: String,
    pub tracking_sensitivity: f32,
    pub animation_smoothing: f32,
    pub expressions: Vec<Expression>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Expression {
    pub name: String,
    pub trigger_words: Vec<String>,
    pub animation_file: String,
}

impl Default for ModelConfig {
    fn default() -> Self {
        ModelConfig {
            model_id: "kamen_sparkle_v1".to_string(),
            tracking_sensitivity: 0.8,
            animation_smoothing: 0.6,
            expressions: vec![
                Expression {
                    name: "happy".to_string(),
                    trigger_words: vec!["happy", "excited", "yay"].into_iter().map(String::from).collect(),
                    animation_file: "animations/happy.json".to_string(),
                },
                // Add more expressions as needed
            ],
        }
    }
} 