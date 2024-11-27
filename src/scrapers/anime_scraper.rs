use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::time::Duration;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::PgPool;
use crate::knowledge::base::KnowledgeBaseManager;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeNews {
    pub title: String,
    pub url: String,
    pub category: String,
    pub timestamp: DateTime<Utc>,
    pub summary: String,
    pub related_titles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeArticle {
    pub title: String,
    pub content: String,
    pub category: String,
    pub timestamp: DateTime<Utc>,
    pub tags: Vec<String>,
    pub url: String,
}

pub struct AnimeNewsScraper {
    client: Client,
    base_url: String,
    cache: HashMap<String, AnimeNews>,
    last_scrape: std::time::Instant,
}

impl AnimeNewsScraper {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("KamenSparkle/1.0")
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();

        Self {
            client,
            base_url: "https://www.animenewsnetwork.com".to_string(),
            cache: HashMap::new(),
            last_scrape: std::time::Instant::now(),
        }
    }

    pub async fn scrape_latest_news(&mut self) -> Result<Vec<AnimeNews>, Box<dyn std::error::Error>> {
        // Respect rate limiting
        if self.last_scrape.elapsed() < Duration::from_secs(5) {
            tokio::time::sleep(Duration::from_secs(5)).await;
        }

        let url = format!("{}/news", self.base_url);
        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        let document = Html::parse_document(&html);

        let news_items = self.parse_news_items(&document)?;
        self.last_scrape = std::time::Instant::now();

        Ok(news_items)
    }

    fn parse_news_items(&self, document: &Html) -> Result<Vec<AnimeNews>, Box<dyn std::error::Error>> {
        let news_selector = Selector::parse("div.news_item, div.herald").unwrap();
        let title_selector = Selector::parse("h3, h2").unwrap();
        let timestamp_selector = Selector::parse("time").unwrap();
        let category_selector = Selector::parse("div.topic").unwrap();

        let mut news_items = Vec::new();

        for item in document.select(&news_selector) {
            let title = item
                .select(&title_selector)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            let url = item
                .select(&title_selector)
                .next()
                .and_then(|el| el.select(&Selector::parse("a").unwrap()).next())
                .and_then(|el| el.value().attr("href"))
                .map(|href| format!("{}{}", self.base_url, href))
                .unwrap_or_default();

            let category = item
                .select(&category_selector)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or("General".to_string());

            let timestamp = item
                .select(&timestamp_selector)
                .next()
                .and_then(|el| el.value().attr("datetime"))
                .and_then(|dt| DateTime::parse_from_rfc3339(dt).ok())
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(Utc::now);

            let summary = self.extract_summary(&item);
            let related_titles = self.extract_related_titles(&item);

            let news = AnimeNews {
                title,
                url,
                category,
                timestamp,
                summary,
                related_titles,
            };

            news_items.push(news);
        }

        Ok(news_items)
    }

    fn extract_summary(&self, item: &scraper::element_ref::ElementRef) -> String {
        let summary_selector = Selector::parse("div.preview, div.summary").unwrap();
        item.select(&summary_selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default()
            .trim()
            .to_string()
    }

    fn extract_related_titles(&self, item: &scraper::element_ref::ElementRef) -> Vec<String> {
        let titles_selector = Selector::parse("div.related-titles a").unwrap();
        item.select(&titles_selector)
            .map(|el| el.text().collect::<String>())
            .collect()
    }

    pub async fn update_anime_knowledge(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let news_items = self.scrape_latest_news().await?;

        let pool = PgPool::connect("your_database_url").await?;
        let knowledge_base_manager = KnowledgeBaseManager::new(pool).await;

        for news in news_items {
            let content = serde_json::json!({
                "title": news.title,
                "summary": news.summary,
                "related_titles": news.related_titles,
                "timestamp": news.timestamp,
            });

            knowledge_base_manager.update_topic(&news.category, &content).await?;
        }

        Ok(())
    }

    pub async fn scrape_seasonal_anime(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url = format!("{}/encyclopedia/anime/season", self.base_url);
        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        let document = Html::parse_document(&html);

        let anime_selector = Selector::parse("div.seasonal-anime h3").unwrap();
        let titles: Vec<String> = document
            .select(&anime_selector)
            .map(|el| el.text().collect::<String>())
            .collect();

        Ok(titles)
    }

    pub async fn get_article(&mut self, url: &str) -> Result<AnimeArticle, Box<dyn std::error::Error>> {
        let response = self.client.get(url).send().await?;
        let html = response.text().await?;
        let document = Html::parse_document(&html);

        let title_selector = Selector::parse("h1.page-title").unwrap();
        let content_selector = Selector::parse("div.article-content").unwrap();
        let tags_selector = Selector::parse("div.tags a").unwrap();

        let title = document
            .select(&title_selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();

        let content = document
            .select(&content_selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();

        let tags: Vec<String> = document
            .select(&tags_selector)
            .map(|el| el.text().collect::<String>())
            .collect();

        Ok(AnimeArticle {
            title,
            content,
            category: "Article".to_string(),
            timestamp: Utc::now(),
            tags,
            url: url.to_string(),
        })
    }
}

pub async fn fetch_anime_trends() -> Result<Value, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://api.anime-trends.com/trending")
        .await?
        .json::<Value>()
        .await?;
    Ok(response)
} 