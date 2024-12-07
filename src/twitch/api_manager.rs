use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::moderation::content_filter::{ContentFilter, MusicContentFilter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitchConfig {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_expiry: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamInfo {
    pub title: String,
    pub game_name: String,
    pub viewer_count: u32,
    pub started_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub language: String,
}

pub struct TwitchAPI {
    client: Client,
    config: TwitchConfig,
    base_url: String,
    event_handlers: HashMap<String, Box<dyn Fn(serde_json::Value) + Send + Sync>>,
}

impl TwitchAPI {
    pub fn new(config: TwitchConfig) -> Self {
        let client = Client::builder()
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            config,
            base_url: "https://api.twitch.tv/helix".to_string(),
            event_handlers: HashMap::new(),
        }
    }

    pub async fn authenticate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let auth_url = "https://id.twitch.tv/oauth2/token";
        let params = [
            ("client_id", &self.config.client_id),
            ("client_secret", &self.config.client_secret),
            ("grant_type", &"client_credentials".to_string()),
        ];

        let response: serde_json::Value = self.client
            .post(auth_url)
            .form(&params)
            .send()
            .await?
            .json()
            .await?;

        if let Some(access_token) = response.get("access_token").and_then(|v| v.as_str()) {
            self.config.access_token = Some(access_token.to_string());
            self.config.token_expiry = Some(Utc::now() + chrono::Duration::hours(4));
        }

        Ok(())
    }

    pub async fn update_stream_info(&self, title: &str, game_id: &str, tags: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/channels", self.base_url);
        let body = serde_json::json!({
            "title": title,
            "game_id": game_id,
            "tags": tags,
        });

        self.client
            .patch(&url)
            .header("Client-ID", &self.config.client_id)
            .header("Authorization", format!("Bearer {}", self.config.access_token.as_ref().unwrap()))
            .json(&body)
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_stream_info(&self) -> Result<StreamInfo, Box<dyn std::error::Error>> {
        let url = format!("{}/streams", self.base_url);
        
        let response = self.client
            .get(&url)
            .header("Client-ID", &self.config.client_id)
            .header("Authorization", format!("Bearer {}", self.config.access_token.as_ref().unwrap()))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        if let Some(data) = response.get("data").and_then(|d| d.as_array()) {
            if let Some(stream) = data.first() {
                return Ok(StreamInfo {
                    title: stream["title"].as_str().unwrap_or_default().to_string(),
                    game_name: stream["game_name"].as_str().unwrap_or_default().to_string(),
                    viewer_count: stream["viewer_count"].as_u64().unwrap_or(0) as u32,
                    started_at: DateTime::parse_from_rfc3339(stream["started_at"].as_str().unwrap_or_default())
                        .unwrap_or_default()
                        .with_timezone(&Utc),
                    tags: stream["tags"].as_array()
                        .map(|t| t.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                        .unwrap_or_default(),
                    language: stream["language"].as_str().unwrap_or("en").to_string(),
                });
            }
        }

        Err("Stream not found".into())
    }

    pub async fn create_clip(&self) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}/clips", self.base_url);
        
        let response: serde_json::Value = self.client
            .post(&url)
            .header("Client-ID", &self.config.client_id)
            .header("Authorization", format!("Bearer {}", self.config.access_token.as_ref().unwrap()))
            .send()
            .await?
            .json()
            .await?;

        if let Some(data) = response.get("data").and_then(|d| d.as_array()) {
            if let Some(clip) = data.first() {
                return Ok(clip["id"].as_str().unwrap_or_default().to_string());
            }
        }

        Err("Failed to create clip".into())
    }

    pub async fn create_marker(&self, description: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/streams/markers", self.base_url);
        let body = serde_json::json!({
            "description": description,
        });

        self.client
            .post(&url)
            .header("Client-ID", &self.config.client_id)
            .header("Authorization", format!("Bearer {}", self.config.access_token.as_ref().unwrap()))
            .json(&body)
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_chatters(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url = format!("{}/chat/chatters", self.base_url);
        
        let response: serde_json::Value = self.client
            .get(&url)
            .header("Client-ID", &self.config.client_id)
            .header("Authorization", format!("Bearer {}", self.config.access_token.as_ref().unwrap()))
            .send()
            .await?
            .json()
            .await?;

        if let Some(data) = response.get("data").and_then(|d| d.as_array()) {
            return Ok(data.iter()
                .filter_map(|user| user["user_name"].as_str())
                .map(String::from)
                .collect());
        }

        Ok(Vec::new())
    }

    pub fn register_event_handler<F>(&mut self, event_type: &str, handler: F)
    where
        F: Fn(serde_json::Value) + Send + Sync + 'static,
    {
        self.event_handlers.insert(event_type.to_string(), Box::new(handler));
    }

    pub async fn handle_event(&self, event_type: &str, data: serde_json::Value) {
        if let Some(handler) = self.event_handlers.get(event_type) {
            handler(data);
        }
    }

    pub async fn ensure_stream_compliance(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Check stream title and tags
        let stream_info = self.get_stream_info().await?;
        
        // Verify stream title compliance
        if !is_compliant_title(&stream_info.title) {
            return Err("Stream title violates Twitch guidelines".into());
        }

        // Verify stream tags compliance
        for tag in &stream_info.tags {
            if !is_compliant_tag(tag) {
                return Err(format!("Stream tag '{}' violates Twitch guidelines", tag).into());
            }
        }

        // Check for DMCA music
        if let Some(music_info) = self.get_current_music().await? {
            if !is_dmca_safe_music(&music_info) {
                return Err("Current music may violate DMCA guidelines".into());
            }
        }

        Ok(())
    }

    fn is_compliant_title(title: &str) -> bool {
        // Check against inappropriate content
        !title.contains_inappropriate_content() &&
        // Check against harmful content
        !title.contains_harmful_content() &&
        // Check length and formatting
        title.len() <= 140 &&
        // No excessive caps
        !has_excessive_caps(title)
    }

    fn is_compliant_tag(tag: &str) -> bool {
        // Check against inappropriate content
        !tag.contains_inappropriate_content() &&
        // Check against harmful content  
        !tag.contains_harmful_content() &&
        // Check length
        tag.len() <= 25 &&
        // No spaces or special characters
        tag.chars().all(|c| c.is_alphanumeric() || c == '_')
    }

    fn is_dmca_safe_music(music_info: &MusicInfo) -> bool {
        // Check if music is from approved sources
        music_info.is_from_approved_source() ||
        // Check if we have proper licenses
        music_info.has_valid_license() ||
        // Check if it's royalty free
        music_info.is_royalty_free()
    }
} 