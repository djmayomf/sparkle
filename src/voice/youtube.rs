use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YouTubePlaylist {
    pub playlist_id: String,
    pub title: String,
    pub videos: Vec<YouTubeVideo>,
    pub total_duration: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YouTubeVideo {
    pub video_id: String,
    pub title: String,
    pub duration: i64,
    pub is_karaoke: bool,
    pub thumbnail_url: String,
}

pub struct YouTubeManager {
    client: Client,
    api_key: String,
    cache: HashMap<String, YouTubeVideo>,
}

impl YouTubeManager {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            cache: HashMap::new(),
        }
    }

    pub async fn fetch_karaoke_version(&self, song_title: &str, artist: &str) -> Result<YouTubeVideo, Box<dyn std::error::Error>> {
        let search_query = format!("{} {} karaoke instrumental", song_title, artist);
        let url = format!(
            "https://www.googleapis.com/youtube/v3/search?part=snippet&q={}&type=video&key={}",
            urlencoding::encode(&search_query),
            self.api_key
        );

        let response: serde_json::Value = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        // Get first result that contains "karaoke" or "instrumental"
        if let Some(items) = response["items"].as_array() {
            for item in items {
                let title = item["snippet"]["title"].as_str().unwrap_or_default().to_lowercase();
                if title.contains("karaoke") || title.contains("instrumental") {
                    let video_id = item["id"]["videoId"].as_str().unwrap_or_default();
                    return self.get_video_details(video_id).await;
                }
            }
        }

        Err("No suitable karaoke version found".into())
    }

    async fn get_video_details(&self, video_id: &str) -> Result<YouTubeVideo, Box<dyn std::error::Error>> {
        if let Some(cached) = self.cache.get(video_id) {
            return Ok(cached.clone());
        }

        let url = format!(
            "https://www.googleapis.com/youtube/v3/videos?part=snippet,contentDetails&id={}&key={}",
            video_id,
            self.api_key
        );

        let response: serde_json::Value = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        if let Some(item) = response["items"].as_array().and_then(|items| items.first()) {
            let video = YouTubeVideo {
                video_id: video_id.to_string(),
                title: item["snippet"]["title"].as_str().unwrap_or_default().to_string(),
                duration: parse_duration(item["contentDetails"]["duration"].as_str().unwrap_or_default()),
                is_karaoke: true,
                thumbnail_url: item["snippet"]["thumbnails"]["high"]["url"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
            };

            self.cache.insert(video_id.to_string(), video.clone());
            Ok(video)
        } else {
            Err("Video not found".into())
        }
    }
}

// Update KaraokeManager to include YouTube functionality
impl KaraokeManager {
    pub struct KaraokeManager {
        state: KaraokeState,
        voice_tx: mpsc::Sender<String>,
        tts_config: TtsKaraokeConfig,
        youtube: YouTubeManager,  // Add this field
    }

    pub fn new(voice_tx: mpsc::Sender<String>, youtube_api_key: String) -> Self {
        Self {
            state: KaraokeState {
                is_active: false,
                current_song: None,
                queue: VecDeque::new(),
                is_subathon: false,
            },
            voice_tx,
            tts_config: TtsKaraokeConfig::default(),
            youtube: YouTubeManager::new(youtube_api_key),
        }
    }

    pub async fn queue_song(&mut self, song: KaraokeSong) -> Result<(), Box<dyn std::error::Error>> {
        if !self.state.is_active {
            return Err("Karaoke mode is not active!".into());
        }

        // Find karaoke version on YouTube
        let video = self.youtube.fetch_karaoke_version(&song.title, &song.artist).await?;
        
        // Create song with YouTube info
        let song_with_video = KaraokeSong {
            youtube_id: video.video_id,
            ..song
        };

        self.state.queue.push_back(song_with_video.clone());
        
        self.voice_tx.send(format!(
            "Added {} by {} to the karaoke queue! Found karaoke version: {} 🎵",
            song.title, song.artist, video.title
        )).await?;
        
        Ok(())
    }

    pub async fn perform_current_song(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(song) = &self.state.current_song {
            // Get video details if not already cached
            let video = self.youtube.get_video_details(&song.youtube_id).await?;
            
            // Process lyrics with timing
            let timed_lyrics = self.process_lyrics_with_timing(song).await?;
            
            // Configure voice for this song
            self.apply_singing_voice_config(song);

            // Show video info to AI (internal only)
            println!("Now playing karaoke video: {}", video.title);
            println!("Video duration: {} seconds", video.duration);
            println!("Thumbnail: {}", video.thumbnail_url);

            // Perform the song
            for lyric in timed_lyrics {
                self.display_current_lyric(&lyric);
                tokio::time::sleep(tokio::time::Duration::from_secs_f32(lyric.start_time)).await;
                self.sing_phrase(&lyric).await?;

                if lyric.is_breath_point {
                    tokio::time::sleep(tokio::time::Duration::from_secs_f32(0.2)).await;
                }
            }
        }
        Ok(())
    }
}

// Helper function to parse YouTube duration format
fn parse_duration(duration: &str) -> i64 {
    let mut total = 0;
    let mut current = 0;
    
    for c in duration.chars() {
        match c {
            'P' | 'T' => continue,
            'H' => {
                total += current * 3600;
                current = 0;
            }
            'M' => {
                total += current * 60;
                current = 0;
            }
            'S' => {
                total += current;
                current = 0;
            }
            d if d.is_digit(10) => {
                current = current * 10 + d.to_digit(10).unwrap() as i64;
            }
            _ => {}
        }
    }
    
    total
} 