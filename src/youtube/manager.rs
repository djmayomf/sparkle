use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub duration: String,
    pub thumbnail_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Playlist {
    pub name: String,
    pub videos: Vec<Video>,
    pub current_index: usize,
}

pub struct YouTubeManager {
    client: Client,
    api_key: String,
    playlists: HashMap<String, Playlist>,
    current_playlist: Option<String>,
}

impl YouTubeManager {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            playlists: HashMap::new(),
            current_playlist: None,
        }
    }

    pub async fn search_videos(&self, query: &str) -> Result<Vec<Video>, Box<dyn std::error::Error>> {
        let url = format!(
            "https://www.googleapis.com/youtube/v3/search?part=snippet&q={}&type=video&key={}",
            query, self.api_key
        );

        let response = self.client.get(&url).send().await?;
        let results: serde_json::Value = response.json().await?;
        
        // Parse results into Video structs
        let mut videos = Vec::new();
        if let Some(items) = results["items"].as_array() {
            for item in items {
                if let (Some(id), Some(title)) = (
                    item["id"]["videoId"].as_str(),
                    item["snippet"]["title"].as_str(),
                ) {
                    videos.push(Video {
                        id: id.to_string(),
                        title: title.to_string(),
                        duration: "".to_string(), // Would need additional API call to get duration
                        thumbnail_url: item["snippet"]["thumbnails"]["default"]["url"]
                            .as_str()
                            .unwrap_or("")
                            .to_string(),
                    });
                }
            }
        }

        Ok(videos)
    }

    pub async fn create_playlist(&mut self, name: String, videos: Vec<Video>) {
        let playlist = Playlist {
            name: name.clone(),
            videos,
            current_index: 0,
        };
        
        self.playlists.insert(name, playlist);
    }

    pub fn next_video(&mut self) -> Option<&Video> {
        if let Some(playlist_name) = &self.current_playlist {
            if let Some(playlist) = self.playlists.get_mut(playlist_name) {
                playlist.current_index = (playlist.current_index + 1) % playlist.videos.len();
                return playlist.videos.get(playlist.current_index);
            }
        }
        None
    }

    pub fn previous_video(&mut self) -> Option<&Video> {
        if let Some(playlist_name) = &self.current_playlist {
            if let Some(playlist) = self.playlists.get_mut(playlist_name) {
                if playlist.current_index > 0 {
                    playlist.current_index -= 1;
                } else {
                    playlist.current_index = playlist.videos.len() - 1;
                }
                return playlist.videos.get(playlist.current_index);
            }
        }
        None
    }
} 