use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::time::Duration;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct DarkReadingArticle {
    pub title: String,
    pub url: String,
    pub category: String,
    pub timestamp: DateTime<Utc>,
    pub summary: String,
    pub author: String,
    pub tags: Vec<String>,
    pub related_topics: Vec<String>,
}

pub struct DarkReadingScraper {
    client: Client,
    base_url: String,
    cache: HashMap<String, DarkReadingArticle>,
    last_scrape: std::time::Instant,
}

impl DarkReadingScraper {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("KamenSparkle/1.0")
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();

        Self {
            client,
            base_url: "https://www.darkreading.com".to_string(),
            cache: HashMap::new(),
            last_scrape: std::time::Instant::now(),
        }
    }

    pub async fn scrape_latest_articles(&mut self) -> Result<Vec<DarkReadingArticle>, Box<dyn std::error::Error>> {
        // Respect rate limiting
        if self.last_scrape.elapsed() < Duration::from_secs(5) {
            tokio::time::sleep(Duration::from_secs(5)).await;
        }

        let categories = vec![
            "application-security",
            "cloud-security", 
            "endpoint-security",
            "threat-intelligence",
            "vulnerabilities-threats",
            "ics-ot-security"
        ];

        let mut articles = Vec::new();
        for category in categories {
            let url = format!("{}/{}", self.base_url, category);
            let response = self.client.get(&url).send().await?;
            let html = response.text().await?;
            let document = Html::parse_document(&html);
            
            articles.extend(self.parse_articles(&document, category)?);
            
            // Respect rate limiting between category requests
            tokio::time::sleep(Duration::from_secs(2)).await;
        }

        self.last_scrape = std::time::Instant::now();
        Ok(articles)
    }

    pub async fn update_security_knowledge(&mut self, knowledge_base: &mut crate::knowledge::base::KnowledgeBase) -> Result<(), Box<dyn std::error::Error>> {
        let articles = self.scrape_latest_articles().await?;

        for article in articles {
            let info = crate::knowledge::base::SecurityInfo {
                topic: article.title,
                description: article.summary,
                difficulty: self.determine_difficulty(&article),
                real_world_examples: vec![format!("Recent article: {}", article.url)],
                best_practices: self.extract_best_practices(&article.summary),
                tools: self.extract_tools(&article.summary),
                resources: vec![article.url],
            };

            knowledge_base.add_security_info(&article.category, info);
        }

        Ok(())
    }
} 