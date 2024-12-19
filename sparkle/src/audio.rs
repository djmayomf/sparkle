use crate::error::{AppError, Result};
use reqwest::Client;
use serde_json::json;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

pub struct AudioTranscriber {
    client: Client,
    api_key: String,
}

impl AudioTranscriber {
    pub async fn new() -> Result<Self> {
        let api_key = std::env::var("GOOGLE_API_KEY")
            .map_err(|_| AppError::Config("GOOGLE_API_KEY must be set".to_string()))?;

        Ok(Self {
            client: Client::new(),
            api_key,
        })
    }

    pub async fn transcribe_audio(&self, audio_data: &[u8]) -> Result<String> {
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

        let transcription = response_json["results"][0]["alternatives"][0]["transcript"]
            .as_str()
            .ok_or_else(|| AppError::Voice("Missing transcript".to_string()))?
            .to_string();

        Ok(transcription)
    }
}

// Re-export the main functionality
pub async fn transcribe_audio(audio_data: &[u8]) -> Result<String> {
    let transcriber = AudioTranscriber::new().await?;
    transcriber.transcribe_audio(audio_data).await
} 