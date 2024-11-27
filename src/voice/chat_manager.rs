use crate::error::{AppError, Result};
use reqwest::Client;
use serde_json::json;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

pub struct VoiceChatManager {
    client: Client,
    api_key: String,
}

#[derive(Debug)]
pub struct VoiceMessage {
    pub content: String,
    pub confidence: f32,
}

impl VoiceChatManager {
    pub async fn new() -> Result<Self> {
        let api_key = std::env::var("GOOGLE_API_KEY")
            .map_err(|_| AppError::Config("GOOGLE_API_KEY must be set".to_string()))?;

        Ok(Self {
            client: Client::new(),
            api_key,
        })
    }

    pub async fn process_audio(&self, audio_data: &[u8]) -> Result<VoiceMessage> {
        let request_body = json!({
            "config": {
                "encoding": "LINEAR16",
                "sampleRateHertz": 16000,
                "languageCode": "en-US",
                "model": "default",
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

        let result = response_json["results"][0]["alternatives"][0]
            .as_object()
            .ok_or_else(|| AppError::Voice("Invalid response format".to_string()))?;

        let content = result["transcript"]
            .as_str()
            .ok_or_else(|| AppError::Voice("Missing transcript".to_string()))?
            .to_string();

        let confidence = result["confidence"]
            .as_f64()
            .ok_or_else(|| AppError::Voice("Missing confidence".to_string()))? as f32;

        Ok(VoiceMessage {
            content,
            confidence,
        })
    }
} 