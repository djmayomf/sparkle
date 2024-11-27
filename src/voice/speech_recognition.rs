use crate::error::{AppError, Result};
use reqwest::Client;
use serde_json::json;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

pub struct SpeechRecognizer {
    client: Client,
    api_key: String,
}

#[derive(Debug)]
pub enum CommandType {
    GameControl,
    StreamControl,
    SystemControl,
}

#[derive(Debug)]
pub struct VoiceCommand {
    pub command_type: CommandType,
    pub content: String,
    pub confidence: f32,
}

impl SpeechRecognizer {
    pub async fn new() -> Result<Self> {
        let api_key = std::env::var("GOOGLE_API_KEY")
            .map_err(|_| AppError::Config("GOOGLE_API_KEY must be set".to_string()))?;

        Ok(Self {
            client: Client::new(),
            api_key,
        })
    }

    pub async fn recognize_command(&self, audio_data: &[u8]) -> Result<Option<VoiceCommand>> {
        let request_body = json!({
            "config": {
                "encoding": "LINEAR16",
                "sampleRateHertz": 16000,
                "languageCode": "en-US",
                "model": "command_and_search",
                "enableAutomaticPunctuation": true,
            },
            "audio": {
                "content": STANDARD.encode(audio_data)
            }
        });

        let response = self.client
            .post(&format!(
                "https://speech.googleapis.com/v1/speech:recognize?key={}",
                self.api_key
            ))
            .json(&request_body)
            .send()
            .await
            .map_err(|e| AppError::Voice(e.to_string()))?;

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::Voice(e.to_string()))?;

        if let Some(result) = response_json["results"].get(0) {
            if let Some(alt) = result["alternatives"].get(0) {
                let content = alt["transcript"]
                    .as_str()
                    .ok_or_else(|| AppError::Voice("Missing transcript".to_string()))?
                    .to_string();
                
                let confidence = alt["confidence"]
                    .as_f64()
                    .ok_or_else(|| AppError::Voice("Missing confidence".to_string()))? as f32;

                let command_type = if content.contains("game") {
                    CommandType::GameControl
                } else if content.contains("stream") {
                    CommandType::StreamControl
                } else if content.contains("system") {
                    CommandType::SystemControl
                } else {
                    return Ok(None);
                };

                return Ok(Some(VoiceCommand {
                    command_type,
                    content,
                    confidence,
                }));
            }
        }

        Ok(None)
    }
} 