use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::time::Duration;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeckInfo {
    pub name: String,
    pub tier: DeckTier,
    pub archetype: String,
    pub play_style: String,
    pub core_cards: Vec<String>,
    pub tech_choices: Vec<String>,
    pub counters: Vec<String>,
    pub difficulty: u8, // 1-10
    pub last_updated: DateTime<Utc>,
    pub deck_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeckTier {
    God,
    Tier1,
    Tier2,
    Tier3,
    Rogue,
    Ojama,
}

pub struct MasterDuelScraper {
    client: Client,
    base_url: String,
    cache: HashMap<String, DeckInfo>,
    last_scrape: DateTime<Utc>,
}

impl MasterDuelScraper {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("KamenSparkle/1.0")
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();

        Self {
            client,
            base_url: "https://game8.co/games/Yu-Gi-Oh-Master-Duel/archives/355397".to_string(),
            cache: HashMap::new(),
            last_scrape: Utc::now(),
        }
    }

    pub async fn scrape_tier_list(&mut self) -> Result<HashMap<DeckTier, Vec<DeckInfo>>, Box<dyn std::error::Error>> {
        let mut tier_list = HashMap::new();

        // Respect rate limiting
        let time_since_last = Utc::now() - self.last_scrape;
        if time_since_last < chrono::Duration::hours(24) {
            return Ok(self.group_cached_decks_by_tier());
        }

        let html = self.client.get(&self.base_url).send().await?.text().await?;
        let document = Html::parse_document(&html);

        // Scrape each tier section
        self.scrape_tier(&document, "God Tier", DeckTier::God, &mut tier_list).await?;
        self.scrape_tier(&document, "Tier 1", DeckTier::Tier1, &mut tier_list).await?;
        self.scrape_tier(&document, "Tier 2", DeckTier::Tier2, &mut tier_list).await?;
        self.scrape_tier(&document, "Tier 3", DeckTier::Tier3, &mut tier_list).await?;
        self.scrape_tier(&document, "Rogue Tier", DeckTier::Rogue, &mut tier_list).await?;
        self.scrape_tier(&document, "Ojama Tier", DeckTier::Ojama, &mut tier_list).await?;

        self.last_scrape = Utc::now();
        Ok(tier_list)
    }

    async fn scrape_tier(
        &mut self,
        document: &Html,
        tier_name: &str,
        tier: DeckTier,
        tier_list: &mut HashMap<DeckTier, Vec<DeckInfo>>
    ) -> Result<(), Box<dyn std::error::Error>> {
        let tier_selector = Selector::parse(&format!("h3:contains('{}')", tier_name)).unwrap();
        let deck_selector = Selector::parse("table tr td a").unwrap();

        if let Some(tier_section) = document.select(&tier_selector).next() {
            let mut decks = Vec::new();

            // Find all deck links in this tier section
            for deck_link in document.select(&deck_selector) {
                if let Some(href) = deck_link.value().attr("href") {
                    if href.contains("/archives/") {
                        // Scrape individual deck page
                        if let Ok(deck_info) = self.scrape_deck_page(href).await {
                            decks.push(deck_info);
                            
                            // Respect rate limiting between deck pages
                            tokio::time::sleep(Duration::from_secs(2)).await;
                        }
                    }
                }
            }

            tier_list.insert(tier, decks);
        }

        Ok(())
    }

    async fn scrape_deck_page(&mut self, url: &str) -> Result<DeckInfo, Box<dyn std::error::Error>> {
        // Check cache first
        if let Some(cached) = self.cache.get(url) {
            return Ok(cached.clone());
        }

        let full_url = if url.starts_with("http") {
            url.to_string()
        } else {
            format!("https://game8.co{}", url)
        };

        let html = self.client.get(&full_url).send().await?.text().await?;
        let document = Html::parse_document(&html);

        let deck_info = self.parse_deck_page(&document, &full_url)?;
        self.cache.insert(url.to_string(), deck_info.clone());

        Ok(deck_info)
    }

    fn parse_deck_page(&self, document: &Html, url: &str) -> Result<DeckInfo, Box<dyn std::error::Error>> {
        let title_selector = Selector::parse("h1.page-title").unwrap();
        let content_selector = Selector::parse("div.a-content").unwrap();
        let card_selector = Selector::parse("table.center tr td").unwrap();

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

        // Extract core cards
        let core_cards: Vec<String> = document
            .select(&card_selector)
            .filter_map(|el| {
                let text = el.text().collect::<String>();
                if !text.trim().is_empty() {
                    Some(text.trim().to_string())
                } else {
                    None
                }
            })
            .collect();

        Ok(DeckInfo {
            name: title,
            tier: self.determine_tier(&content),
            archetype: self.extract_archetype(&content),
            play_style: self.extract_play_style(&content),
            core_cards,
            tech_choices: self.extract_tech_choices(&content),
            counters: self.extract_counters(&content),
            difficulty: self.determine_difficulty(&content),
            last_updated: Utc::now(),
            deck_url: url.to_string(),
        })
    }

    fn determine_tier(&self, content: &str) -> DeckTier {
        if content.contains("God Tier") {
            DeckTier::God
        } else if content.contains("Tier 1") {
            DeckTier::Tier1
        } else if content.contains("Tier 2") {
            DeckTier::Tier2
        } else if content.contains("Tier 3") {
            DeckTier::Tier3
        } else if content.contains("Rogue") {
            DeckTier::Rogue
        } else {
            DeckTier::Ojama
        }
    }

    fn group_cached_decks_by_tier(&self) -> HashMap<DeckTier, Vec<DeckInfo>> {
        let mut grouped = HashMap::new();
        
        for deck in self.cache.values() {
            grouped
                .entry(deck.tier.clone())
                .or_insert_with(Vec::new)
                .push(deck.clone());
        }
        
        grouped
    }

    pub async fn update_yugioh_knowledge(&mut self, knowledge_base: &mut crate::knowledge::base::KnowledgeBase) -> Result<(), Box<dyn std::error::Error>> {
        let tier_list = self.scrape_tier_list().await?;

        for (tier, decks) in tier_list {
            for deck in decks {
                let info = crate::knowledge::base::YugiohInfo {
                    deck_name: deck.name,
                    tier: format!("{:?}", tier),
                    archetype: deck.archetype,
                    play_style: deck.play_style,
                    core_cards: deck.core_cards,
                    tech_choices: deck.tech_choices,
                    counters: deck.counters,
                    last_updated: deck.last_updated,
                };

                knowledge_base.add_yugioh_info(&format!("{:?}", tier), info);
            }
        }

        Ok(())
    }

    // Helper methods for parsing deck information
    fn extract_archetype(&self, content: &str) -> String {
        // Extract archetype information from content
        "Unknown Archetype".to_string()
    }

    fn extract_play_style(&self, content: &str) -> String {
        // Extract play style information from content
        "Unknown Play Style".to_string()
    }

    fn extract_tech_choices(&self, content: &str) -> Vec<String> {
        // Extract tech choices from content
        Vec::new()
    }

    fn extract_counters(&self, content: &str) -> Vec<String> {
        // Extract counter cards/strategies from content
        Vec::new()
    }

    fn determine_difficulty(&self, content: &str) -> u8 {
        // Determine deck difficulty from content
        5
    }
} 