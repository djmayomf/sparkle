use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct YouTubeSearchResponse {
    kind: String,
    etag: String,
    next_page_token: Option<String>,
    prev_page_token: Option<String>,
    region_code: String,
    page_info: PageInfo,
    items: Vec<SearchResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageInfo {
    total_results: i32,
    results_per_page: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    kind: String,
    etag: String,
    id: ResourceId,
    snippet: SearchResultSnippet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceId {
    kind: String,
    video_id: Option<String>,
    channel_id: Option<String>,
    playlist_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResultSnippet {
    published_at: String,
    channel_id: String,
    title: String,
    description: String,
    thumbnails: Thumbnails,
    channel_title: String,
    live_broadcast_content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thumbnails {
    default: Option<Thumbnail>,
    medium: Option<Thumbnail>,
    high: Option<Thumbnail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thumbnail {
    url: String,
    width: i32,
    height: i32,
}

pub struct YouTubeSearcher {
    api_key: String,
    client: Client,
}

impl YouTubeSearcher {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    pub async fn search(&self, query: &str, max_results: i32) -> Result<YouTubeSearchResponse, Box<dyn Error>> {
        let url = format!(
            "https://www.googleapis.com/youtube/v3/search?part=snippet&q={}&maxResults={}&key={}",
            urlencoding::encode(query),
            max_results,
            self.api_key
        );

        let response = self.client
            .get(&url)
            .send()
            .await?
            .json::<YouTubeSearchResponse>()
            .await?;

        Ok(response)
    }

    pub async fn search_videos(&self, query: &str, max_results: i32) -> Result<YouTubeSearchResponse, Box<dyn Error>> {
        let url = format!(
            "https://www.googleapis.com/youtube/v3/search?part=snippet&type=video&q={}&maxResults={}&key={}",
            urlencoding::encode(query),
            max_results,
            self.api_key
        );

        let response = self.client
            .get(&url)
            .send()
            .await?
            .json::<YouTubeSearchResponse>()
            .await?;

        Ok(response)
    }

    pub async fn search_with_filters(&self, params: SearchParams) -> Result<YouTubeSearchResponse, Box<dyn Error>> {
        let mut url = format!(
            "https://www.googleapis.com/youtube/v3/search?part=snippet&key={}",
            self.api_key
        );

        // Add optional parameters
        if let Some(q) = params.query {
            url.push_str(&format!("&q={}", urlencoding::encode(&q)));
        }
        if let Some(max_results) = params.max_results {
            url.push_str(&format!("&maxResults={}", max_results));
        }
        if let Some(order) = params.order {
            url.push_str(&format!("&order={}", order));
        }
        if let Some(page_token) = params.page_token {
            url.push_str(&format!("&pageToken={}", page_token));
        }
        if let Some(type_) = params.type_ {
            url.push_str(&format!("&type={}", type_));
        }
        if let Some(video_duration) = params.video_duration {
            url.push_str(&format!("&videoDuration={}", video_duration));
        }

        let response = self.client
            .get(&url)
            .send()
            .await?
            .json::<YouTubeSearchResponse>()
            .await?;

        Ok(response)
    }
}

#[derive(Debug, Default)]
pub struct SearchParams {
    pub query: Option<String>,
    pub max_results: Option<i32>,
    pub order: Option<String>,
    pub page_token: Option<String>,
    pub type_: Option<String>,
    pub video_duration: Option<String>,
    pub region_code: Option<String>,
    pub relevance_language: Option<String>,
    pub safe_search: Option<String>,
    pub video_caption: Option<String>,
    pub video_definition: Option<String>,
    pub video_dimension: Option<String>,
    pub video_embeddable: Option<String>,
}

impl SearchParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_query(mut self, query: &str) -> Self {
        self.query = Some(query.to_string());
        self
    }

    pub fn with_max_results(mut self, max_results: i32) -> Self {
        self.max_results = Some(max_results);
        self
    }

    pub fn with_order(mut self, order: &str) -> Self {
        self.order = Some(order.to_string());
        self
    }
} 