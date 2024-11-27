use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokuArticle {
    pub title: String,
    pub summary: String,
    pub categories: Vec<String>,
    pub related_pages: Vec<String>,
    pub fun_facts: Vec<String>,
    pub characters: Vec<String>,
    pub url: String,
}

pub struct TokuScraper {
    client: Client,
    base_url: String,
    cache: HashMap<String, TokuArticle>,
    last_scrape: std::time::Instant,
}

impl TokuScraper {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("KamenSparkle/1.0")
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();

        Self {
            client,
            base_url: "https://tokusatsu.fandom.com/wiki".to_string(),
            cache: HashMap::new(),
            last_scrape: std::time::Instant::now(),
        }
    }

    pub async fn scrape_series(&mut self, series_name: &str) -> Result<TokuArticle, Box<dyn std::error::Error>> {
        // Respect rate limiting
        if self.last_scrape.elapsed() < Duration::from_secs(1) {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        // Check cache first
        if let Some(cached) = self.cache.get(series_name) {
            return Ok(cached.clone());
        }

        let url = format!("{}/{}", self.base_url, series_name.replace(" ", "_"));
        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        let document = Html::parse_document(&html);

        let article = self.parse_article(&document, &url)?;
        self.cache.insert(series_name.to_string(), article.clone());
        self.last_scrape = std::time::Instant::now();

        Ok(article)
    }

    fn parse_article(&self, document: &Html, url: &str) -> Result<TokuArticle, Box<dyn std::error::Error>> {
        // Selectors for different parts of the page
        let title_selector = Selector::parse("h1.page-header__title").unwrap();
        let summary_selector = Selector::parse("div.mw-parser-output > p").unwrap();
        let category_selector = Selector::parse("div.page-header__categories a").unwrap();
        let character_selector = Selector::parse("div#Characters li, div#Cast li").unwrap();

        // Extract title
        let title = document
            .select(&title_selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();

        // Extract summary (first non-empty paragraph)
        let summary = document
            .select(&summary_selector)
            .find(|el| !el.text().collect::<String>().trim().is_empty())
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();

        // Extract categories
        let categories = document
            .select(&category_selector)
            .map(|el| el.text().collect::<String>())
            .collect();

        // Extract characters/cast
        let characters = document
            .select(&character_selector)
            .map(|el| el.text().collect::<String>())
            .collect();

        // Extract fun facts from trivia section
        let fun_facts = self.extract_fun_facts(document);

        // Extract related pages
        let related_pages = self.extract_related_pages(document);

        Ok(TokuArticle {
            title,
            summary,
            categories,
            related_pages,
            fun_facts,
            characters,
            url: url.to_string(),
        })
    }

    fn extract_fun_facts(&self, document: &Html) -> Vec<String> {
        let trivia_selector = Selector::parse("div#Trivia li, div#Notes li, div#Behind_the_scenes li").unwrap();
        document
            .select(&trivia_selector)
            .map(|el| el.text().collect::<String>())
            .collect()
    }

    fn extract_related_pages(&self, document: &Html) -> Vec<String> {
        let link_selector = Selector::parse("div.mw-parser-output a[href^='/wiki/']").unwrap();
        document
            .select(&link_selector)
            .map(|el| el.text().collect::<String>())
            .collect()
    }

    pub async fn update_knowledge_base(&mut self, knowledge_base: &mut crate::knowledge::base::KnowledgeBase) -> Result<(), Box<dyn std::error::Error>> {
        // List of major tokusatsu series to scrape
        let series = vec![
            "Kamen_Rider_Series",
            "Super_Sentai",
            "Ultraman",
            "GARO",
            "Metal_Heroes",
        ];

        for series_name in series {
            if let Ok(article) = self.scrape_series(series_name).await {
                // Convert scraped data to TokusatsuInfo format
                let info = crate::knowledge::base::TokusatsuInfo {
                    title: article.title,
                    description: article.summary,
                    year: self.extract_year(&article.summary),
                    notable_characters: article.characters,
                    fun_facts: article.fun_facts,
                    related_series: article.related_pages,
                };

                // Update knowledge base
                knowledge_base.add_tokusatsu_info(series_name, info);
            }

            // Respect rate limiting
            tokio::time::sleep(Duration::from_secs(2)).await;
        }

        Ok(())
    }

    fn extract_year(&self, text: &str) -> u32 {
        // Try to find a year in the text (4 digits starting with 19 or 20)
        let year_regex = regex::Regex::new(r"(19|20)\d{2}").unwrap();
        if let Some(capture) = year_regex.find(text) {
            capture.as_str().parse().unwrap_or(0)
        } else {
            0
        }
    }
} 