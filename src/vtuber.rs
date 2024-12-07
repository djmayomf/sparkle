use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::{Client, ClientBuilder};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};
use tokio::time::sleep;
use url::Url;
use std::error::Error;

// Custom error type for VTuber scraping
#[derive(Debug)]
pub enum VTuberScraperError {
    NetworkError(reqwest::Error),
    ParseError(String),
    RateLimitExceeded,
    RobotsDisallowed(String),
    ValidationError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VTuberInfo {
    name: String,
    japanese_name: Option<String>,
    debut_date: Option<DateTime<Utc>>,
    graduation_date: Option<DateTime<Utc>>,
    agency: Option<String>,
    languages: Vec<String>,
    active_platforms: Vec<String>,
    description: String,
    personality_traits: Vec<String>,
    avatar_description: String,
    social_links: HashMap<String, String>,
    is_graduated: bool,
    previous_personas: Vec<String>,
    achievements: Vec<String>,
    notable_collabs: Vec<String>,
    tags: Vec<String>,
    last_updated: DateTime<Utc>,
}

#[derive(Debug)]
pub struct VTuberScraper {
    client: Client,
    rate_limiter: RateLimiter,
    robots_checker: RobotsChecker,
    logger: ScraperLogger,
}

#[derive(Debug)]
struct RateLimiter {
    requests_per_minute: u32,
    last_request: DateTime<Utc>,
}

#[derive(Debug)]
struct RobotsChecker {
    allowed_paths: HashMap<String, bool>,
    cache_duration: Duration,
    last_checked: HashMap<String, DateTime<Utc>>,
}

#[derive(Debug)]
struct ScraperLogger {
    log_file: std::fs::File,
}

impl VTuberScraper {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let client = ClientBuilder::new()
            .user_agent("RespectfulVTuberBot/1.0 (research purposes; respect for idol culture)")
            .timeout(Duration::from_secs(30))
            .build()?;

        let rate_limiter = RateLimiter {
            requests_per_minute: 10, // Respectful rate limit
            last_request: Utc::now(),
        };

        let robots_checker = RobotsChecker {
            allowed_paths: HashMap::new(),
            cache_duration: Duration::from_secs(3600), // Cache robots.txt for 1 hour
            last_checked: HashMap::new(),
        };

        let logger = ScraperLogger {
            log_file: std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("vtuber_scraper.log")?,
        };

        Ok(Self {
            client,
            rate_limiter,
            robots_checker,
            logger,
        })
    }

    pub async fn scrape_vtuber_wiki(&mut self) -> Result<Vec<VTuberInfo>, VTuberScraperError> {
        let base_urls = vec![
            "https://virtualyoutuber.fandom.com/wiki/Category:English",
            "https://virtualyoutuber.fandom.com/wiki/Category:Japanese",
            "https://virtualyoutuber.fandom.com/wiki/Category:Spanish",
            "https://virtualyoutuber.fandom.com/wiki/Category:German",
        ];

        let mut all_vtubers = Vec::new();

        for url in base_urls {
            // Check robots.txt first
            self.robots_checker.check_url(url).await?;

            // Apply rate limiting
            self.rate_limiter.wait_if_needed().await?;

            // Scrape category page
            let vtubers = self.scrape_category_page(url).await?;
            all_vtubers.extend(vtubers);
        }

        Ok(all_vtubers)
    }

    async fn scrape_category_page(&self, url: &str) -> Result<Vec<VTuberInfo>, VTuberScraperError> {
        let mut vtubers = Vec::new();
        let mut current_page = url.to_string();

        loop {
            let html = self.fetch_page(&current_page).await?;
            let document = Html::parse_document(&html);

            // Extract VTuber links from category page
            let vtuber_links = self.extract_vtuber_links(&document)?;

            for link in vtuber_links {
                // Rate limiting between individual VTuber pages
                self.rate_limiter.wait_if_needed().await?;

                match self.scrape_vtuber_page(&link).await {
                    Ok(vtuber) => vtubers.push(vtuber),
                    Err(e) => {
                        self.logger.log_error(&format!(
                            "Error scraping VTuber page {}: {:?}", 
                            link, e
                        )).await?;
                        continue;
                    }
                }
            }

            // Check for next page
            if let Some(next_page) = self.extract_next_page_link(&document) {
                current_page = next_page;
            } else {
                break;
            }
        }

        Ok(vtubers)
    }

    async fn scrape_vtuber_page(&self, url: &str) -> Result<VTuberInfo, VTuberScraperError> {
        let html = self.fetch_page(url).await?;
        let document = Html::parse_document(&html);

        // Extract VTuber information with respect to graduation status
        let mut vtuber = self.extract_vtuber_info(&document)?;

        // Check for graduation status and handle appropriately
        if self.is_graduated(&document) {
            vtuber.is_graduated = true;
            vtuber.graduation_date = self.extract_graduation_date(&document)?;
            
            // Handle previous personas with respect
            vtuber.previous_personas = self.extract_previous_personas(&document)?;
        }

        self.validate_vtuber_info(&vtuber)?;
        Ok(vtuber)
    }

    fn validate_vtuber_info(&self, vtuber: &VTuberInfo) -> Result<(), VTuberScraperError> {
        if vtuber.name.is_empty() {
            return Err(VTuberScraperError::ValidationError(
                "VTuber name cannot be empty".to_string()
            ));
        }

        // Additional validation rules
        Ok(())
    }
}

impl RateLimiter {
    async fn wait_if_needed(&self) -> Result<(), VTuberScraperError> {
        let elapsed = Utc::now() - self.last_request;
        let min_interval = Duration::from_secs(60) / self.requests_per_minute;

        if elapsed < min_interval {
            sleep(min_interval - elapsed.to_std().unwrap()).await;
        }

        Ok(())
    }
}

impl RobotsChecker {
    async fn check_url(&mut self, url: &str) -> Result<(), VTuberScraperError> {
        let domain = Url::parse(url)
            .map_err(|e| VTuberScraperError::ParseError(e.to_string()))?
            .host_str()
            .ok_or_else(|| VTuberScraperError::ParseError("Invalid URL".to_string()))?
            .to_string();

        // Check cache first
        if let Some(last_checked) = self.last_checked.get(&domain) {
            if Utc::now() - *last_checked < self.cache_duration {
                return if *self.allowed_paths.get(&domain).unwrap_or(&false) {
                    Ok(())
                } else {
                    Err(VTuberScraperError::RobotsDisallowed(domain))
                };
            }
        }

        // Fetch and parse robots.txt
        let robots_url = format!("https://{}/robots.txt", domain);
        // Implementation for robots.txt checking
        
        Ok(())
    }
}

impl ScraperLogger {
    async fn log_error(&self, message: &str) -> Result<(), VTuberScraperError> {
        use std::io::Write;
        writeln!(
            self.log_file,
            "[{}] ERROR: {}",
            Utc::now().format("%Y-%m-%d %H:%M:%S"),
            message
        ).map_err(|e| VTuberScraperError::ParseError(e.to_string()))?;
        Ok(())
    }
} 