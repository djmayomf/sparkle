use crate::security::knowledge_base::{SecurityKnowledgeBase, SecurityLevel};
use crate::tts::model::TTSModel;
use crate::emotions::adapter::EmotionalAdapter;
use tokio::sync::mpsc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamContext {
    pub current_topic: Option<String>,
    pub stream_type: StreamType,
    pub interaction_level: f32,
    pub last_responses: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum StreamType {
    Gaming,
    Security,
    Casual,
    Tutorial,
}

pub struct InteractionHandler {
    security_knowledge: SecurityKnowledgeBase,
    tts_model: TTSModel,
    emotional_adapter: EmotionalAdapter,
    context: StreamContext,
    message_tx: mpsc::Sender<String>,
}

impl InteractionHandler {
    pub fn new(
        security_knowledge: SecurityKnowledgeBase,
        tts_model: TTSModel,
        emotional_adapter: EmotionalAdapter,
        message_tx: mpsc::Sender<String>,
    ) -> Self {
        Self {
            security_knowledge,
            tts_model,
            emotional_adapter,
            context: StreamContext {
                current_topic: None,
                stream_type: StreamType::Casual,
                interaction_level: 0.5,
                last_responses: Vec::new(),
            },
            message_tx,
        }
    }

    pub async fn handle_message(&mut self, user: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Check for security-related questions
        if self.context.stream_type == StreamType::Security {
            if let Some(response) = self.handle_security_question(message).await? {
                self.respond_with_security_info(&response).await?;
                return Ok(());
            }
        }

        // Handle general interaction
        let response = self.generate_contextual_response(user, message).await?;
        self.send_response(&response).await?;

        Ok(())
    }

    async fn handle_security_question(&self, message: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        // Keywords for detecting security-related questions
        let security_keywords = [
            "vulnerability", "exploit", "security", "hack", "protection",
            "firewall", "encryption", "malware", "virus", "breach",
        ];

        if security_keywords.iter().any(|&keyword| message.to_lowercase().contains(keyword)) {
            if let Some(topic) = self.identify_security_topic(message) {
                if let Some(topic_info) = self.security_knowledge.get_topic_info(&topic) {
                    let response = format!(
                        "Great question about {}! 💻 {}\n\nSome real-world examples:\n{}",
                        topic_info.name,
                        topic_info.description,
                        topic_info.real_world_examples.join("\n- ")
                    );
                    return Ok(Some(response));
                }
            }
        }

        Ok(None)
    }

    fn identify_security_topic(&self, message: &str) -> Option<String> {
        // Add logic to identify specific security topics from the message
        // This could be enhanced with NLP for better topic detection
        if message.contains("network") {
            Some("network_basics".to_string())
        } else if message.contains("web") {
            Some("web_security".to_string())
        } else {
            None
        }
    }

    async fn respond_with_security_info(&mut self, info: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Split long responses into digestible chunks
        for chunk in info.chars().collect::<Vec<char>>().chunks(200) {
            let chunk_text = chunk.iter().collect::<String>();
            self.tts_model.speak(&chunk_text).await?;
            self.message_tx.send(chunk_text).await?;
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        Ok(())
    }

    async fn generate_contextual_response(&mut self, user: &str, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        let base_response = match self.context.stream_type {
            StreamType::Gaming => format!("Hey {}! Thanks for watching the gaming stream! (◕‿◕✿)", user),
            StreamType::Security => format!("Welcome {}! Feel free to ask any security questions! 💻", user),
            StreamType::Tutorial => format!("Hi {}! Hope you're learning something new! 📚", user),
            StreamType::Casual => format!("Hello {}! Great to see you! (｡♥‿♥｡)", user),
        };

        // Adapt response based on emotional state
        let adapted_response = self.emotional_adapter.adapt_response(&base_response, 0.7);
        Ok(adapted_response)
    }

    async fn send_response(&mut self, response: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.message_tx.send(response.to_string()).await?;
        self.tts_model.speak(response).await?;
        Ok(())
    }

    pub fn set_stream_type(&mut self, stream_type: StreamType) {
        self.context.stream_type = stream_type;
    }
} 