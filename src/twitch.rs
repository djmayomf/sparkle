use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use dotenv::dotenv;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
struct TwitchAuthResponse {
    access_token: String,
    expires_in: i32,
    token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TwitchStream {
    user_id: String,
    user_login: String,
    user_name: String,
    game_name: String,
    title: String,
    started_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TwitchUserInfo {
    id: String,
    login: String,
    display_name: String,
    description: String,
    profile_image_url: String,
    broadcaster_type: String,
}

pub struct TwitchAPI {
    client: Client,
    client_id: String,
    client_secret: String,
    access_token: Option<String>,
}

impl TwitchAPI {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();
        
        let client_id = env::var("TWITCH_CLIENT_ID")?;
        let client_secret = env::var("TWITCH_CLIENT_SECRET")?;
        
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Client-ID",
            header::HeaderValue::from_str(&client_id)?
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()?;

        let mut api = TwitchAPI {
            client,
            client_id,
            client_secret,
            access_token: None,
        };

        api.refresh_auth_token().await?;
        Ok(api)
    }

    async fn refresh_auth_token(&mut self) -> Result<(), Box<dyn Error>> {
        let auth_url = format!(
            "https://id.twitch.tv/oauth2/token?client_id={}&client_secret={}&grant_type=client_credentials",
            self.client_id, self.client_secret
        );

        let response: TwitchAuthResponse = self.client
            .post(&auth_url)
            .send()
            .await?
            .json()
            .await?;

        self.access_token = Some(response.access_token);
        Ok(())
    }

    pub async fn follow_user(&self, from_id: &str, to_id: &str) -> Result<(), Box<dyn Error>> {
        let url = "https://api.twitch.tv/helix/users/follows";
        
        let response = self.client
            .post(url)
            .bearer_auth(self.access_token.as_ref().unwrap())
            .json(&serde_json::json!({
                "from_id": from_id,
                "to_id": to_id
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Failed to follow user: {}", response.status()).into());
        }

        Ok(())
    }

    pub async fn get_live_streams(&self, game_id: Option<&str>) -> Result<Vec<StreamerInfo>, Box<dyn Error>> {
        let mut url = String::from("https://api.twitch.tv/helix/streams?first=100");
        if let Some(game) = game_id {
            url.push_str(&format!("&game_id={}", game));
        }

        let response = self.client
            .get(&url)
            .bearer_auth(self.access_token.as_ref().unwrap())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Failed to get streams: {}", response.status()).into());
        }

        let streams: Vec<TwitchStream> = response.json().await?;
        
        Ok(streams
            .into_iter()
            .map(|stream| {
                let started_at = chrono::DateTime::parse_from_rfc3339(&stream.started_at)
                    .unwrap_or_default();
                let uptime = chrono::Utc::now()
                    .signed_duration_since(started_at)
                    .num_minutes() as i32;

                StreamerInfo {
                    username: stream.user_login,
                    display_name: stream.user_name,
                    stream_title: stream.title,
                    category: stream.game_name,
                    uptime_minutes: uptime,
                    is_live: true,
                }
            })
            .collect())
    }

    pub async fn get_user_info(&self, username: &str) -> Result<TwitchUserInfo, Box<dyn Error>> {
        let url = format!(
            "https://api.twitch.tv/helix/users?login={}",
            username
        );

        let response = self.client
            .get(&url)
            .bearer_auth(self.access_token.as_ref().unwrap())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Failed to get user info: {}", response.status()).into());
        }

        let data: serde_json::Value = response.json().await?;
        let user = &data["data"][0];

        Ok(TwitchUserInfo {
            id: user["id"].as_str().unwrap_or_default().to_string(),
            login: user["login"].as_str().unwrap_or_default().to_string(),
            display_name: user["display_name"].as_str().unwrap_or_default().to_string(),
            description: user["description"].as_str().unwrap_or_default().to_string(),
            profile_image_url: user["profile_image_url"].as_str().unwrap_or_default().to_string(),
            broadcaster_type: user["broadcaster_type"].as_str().unwrap_or_default().to_string(),
        })
    }

    pub async fn get_channel_info(&self, broadcaster_id: &str) -> Result<String, Box<dyn Error>> {
        let url = format!(
            "https://api.twitch.tv/helix/channels?broadcaster_id={}",
            broadcaster_id
        );

        let response = self.client
            .get(&url)
            .bearer_auth(self.access_token.as_ref().unwrap())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Failed to get channel info: {}", response.status()).into());
        }

        let data: serde_json::Value = response.json().await?;
        Ok(data["data"][0]["game_name"].as_str().unwrap_or_default().to_string())
    }
} 