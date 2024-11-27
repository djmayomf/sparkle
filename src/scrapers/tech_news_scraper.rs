use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use tokio::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechArticle {
    pub title: String,
    pub url: String,
    pub source: NewsSource,
    pub category: String,
    pub summary: String,
    pub author: String,
    pub published_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NewsSource {
    TheVerge,
    TechNewsWorld,
}

pub struct TechNewsScraper {
    client: Client,
    cache: HashMap<String, TechArticle>,
    last_scrape: std::time::Instant,
}

impl TechNewsScraper {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("KamenSparkle/1.0")
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();

        Self {
            client,
            cache: HashMap::new(),
            last_scrape: std::time::Instant::now(),
        }
    }

    pub async fn scrape_verge(&mut self) -> Result<Vec<TechArticle>, Box<dyn std::error::Error>> {
        // Respect rate limiting
        if self.last_scrape.elapsed() < Duration::from_secs(5) {
            tokio::time::sleep(Duration::from_secs(5)).await;
        }

        let url = "https://www.theverge.com/tech";
        let response = self.client.get(url).send().await?;
        let html = response.text().await?;
        let document = Html::parse_document(&html);

        let article_selector = Selector::parse("article.duet--article--standard").unwrap();
        let title_selector = Selector::parse("h2").unwrap();
        let author_selector = Selector::parse("span.text-gray-31").unwrap();
        let summary_selector = Selector::parse("p.duet--article--standard-article__description").unwrap();

        let mut articles = Vec::new();

        for article in document.select(&article_selector) {
            let title = article
                .select(&title_selector)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            let url = article
                .select(&Selector::parse("a").unwrap())
                .next()
                .and_then(|el| el.value().attr("href"))
                .map(|href| format!("https://www.theverge.com{}", href))
                .unwrap_or_default();

            let author = article
                .select(&author_selector)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            let summary = article
                .select(&summary_selector)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            let tech_article = TechArticle {
                title,
                url,
                source: NewsSource::TheVerge,
                category: "Tech".to_string(),
                summary,
                author,
                published_at: Utc::now(), // Would need to parse actual date
                tags: vec!["tech".to_string()],
            };

            articles.push(tech_article);
        }

        self.last_scrape = std::time::Instant::now();
        Ok(articles)
    }

    pub async fn scrape_technewsworld(&mut self) -> Result<Vec<TechArticle>, Box<dyn std::error::Error>> {
        let url = "https://www.technewsworld.com/";
        let response = self.client.get(url).send().await?;
        let html = response.text().await?;
        let document = Html::parse_document(&html);

        let article_selector = Selector::parse("div.story-wrap").unwrap();
        let title_selector = Selector::parse("h2, h3").unwrap();
        let summary_selector = Selector::parse("div.story-summary").unwrap();

        let mut articles = Vec::new();

        for article in document.select(&article_selector) {
            let title = article
                .select(&title_selector)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            let url = article
                .select(&Selector::parse("a").unwrap())
                .next()
                .and_then(|el| el.value().attr("href"))
                .map(String::from)
                .unwrap_or_default();

            let summary = article
                .select(&summary_selector)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            let tech_article = TechArticle {
                title,
                url,
                source: NewsSource::TechNewsWorld,
                category: "Tech".to_string(),
                summary,
                author: "".to_string(), // Would need additional scraping
                published_at: Utc::now(),
                tags: vec!["tech".to_string()],
            };

            articles.push(tech_article);
        }

        Ok(articles)
    }

    pub async fn get_latest_tech_news(&mut self) -> Result<Vec<TechArticle>, Box<dyn std::error::Error>> {
        let mut all_articles = Vec::new();

        // Get news from The Verge
        if let Ok(verge_articles) = self.scrape_verge().await {
            all_articles.extend(verge_articles);
        }

        // Get news from TechNewsWorld
        if let Ok(tnw_articles) = self.scrape_technewsworld().await {
            all_articles.extend(tnw_articles);
        }

        // Sort by publication date
        all_articles.sort_by(|a, b| b.published_at.cmp(&a.published_at));

        Ok(all_articles)
    }

    pub async fn get_trending_topics(&self, articles: &[TechArticle]) -> Vec<String> {
        let mut topic_counts = HashMap::new();

        for article in articles {
            for tag in &article.tags {
                *topic_counts.entry(tag.clone()).or_insert(0) += 1;
            }

            // Also count words in titles for trending topics
            for word in article.title.split_whitespace() {
                if word.len() > 3 { // Skip short words
                    *topic_counts.entry(word.to_lowercase()).or_insert(0) += 1;
                }
            }
        }

        // Sort topics by frequency
        let mut topics: Vec<_> = topic_counts.into_iter().collect();
        topics.sort_by(|a, b| b.1.cmp(&a.1));

        // Return top 10 trending topics
        topics.into_iter()
            .take(10)
            .map(|(topic, _)| topic)
            .collect()
    }
} 