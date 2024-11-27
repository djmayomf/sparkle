use tokio::time::{interval, Duration};
use crate::error::Result;
use crate::scrapers::{
    security_scraper::SecurityScraper,
    anime_scraper::AnimeNewsScraper,
    youtube_personality_scraper::YouTubePersonalityScraper
};
use crate::database::connection::KnowledgeBaseManager;
use std::sync::Arc;
use sqlx::PgPool;
use tracing::{info, error, warn};

pub struct KnowledgeAutoUpdater {
    security_scraper: SecurityScraper,
    anime_scraper: AnimeNewsScraper,
    personality_scraper: YouTubePersonalityScraper,
    knowledge_base: Arc<KnowledgeBaseManager>,
    update_interval: Duration,
    last_update: std::time::Instant,
}

impl KnowledgeAutoUpdater {
    pub fn new(pool: PgPool) -> Result<Self> {
        Ok(Self {
            security_scraper: SecurityScraper::new()?,
            anime_scraper: AnimeNewsScraper::new(),
            personality_scraper: YouTubePersonalityScraper::new("playlist_id")?,
            knowledge_base: Arc::new(KnowledgeBaseManager::new(pool).await),
            update_interval: Duration::from_secs(300), // 5 minutes default
            last_update: std::time::Instant::now(),
        })
    }

    pub async fn start_auto_updates(&mut self) -> Result<()> {
        info!("Starting knowledge base auto-updater");
        let mut interval = interval(self.update_interval);

        loop {
            interval.tick().await;
            if let Err(e) = self.update_all_knowledge().await {
                error!("Error updating knowledge base: {}", e);
                continue;
            }
            self.last_update = std::time::Instant::now();
            info!("Knowledge base updated successfully");
        }
    }

    async fn update_all_knowledge(&mut self) -> Result<()> {
        // Update security knowledge
        if let Err(e) = self.update_security_knowledge().await {
            warn!("Error updating security knowledge: {}", e);
        }

        // Update anime knowledge
        if let Err(e) = self.update_anime_knowledge().await {
            warn!("Error updating anime knowledge: {}", e);
        }

        // Update personality traits
        if let Err(e) = self.update_personality_knowledge().await {
            warn!("Error updating personality knowledge: {}", e);
        }

        Ok(())
    }

    async fn update_security_knowledge(&mut self) -> Result<()> {
        let security_events = self.security_scraper.get_security_insights().await?;
        
        for event in security_events {
            let content = serde_json::json!({
                "event_type": event.event_type,
                "description": event.description,
                "severity": event.severity,
                "recommendations": event.recommendations,
                "timestamp": chrono::Utc::now(),
            });

            self.knowledge_base.update_topic("security_events", &content).await?;
        }

        Ok(())
    }

    async fn update_anime_knowledge(&mut self) -> Result<()> {
        let news_items = self.anime_scraper.scrape_latest_news().await?;
        
        for news in news_items {
            let content = serde_json::json!({
                "title": news.title,
                "summary": news.summary,
                "related_titles": news.related_titles,
                "timestamp": news.timestamp,
            });

            self.knowledge_base.update_topic(&news.category, &content).await?;
        }

        Ok(())
    }

    async fn update_personality_knowledge(&mut self) -> Result<()> {
        let personality = self.personality_scraper.generate_unique_personality().await?;
        
        let content = serde_json::json!({
            "base_traits": personality.base_traits,
            "unique_quirks": personality.unique_quirks,
            "personality_signature": personality.personality_signature,
            "catchphrases": personality.catchphrases,
            "last_updated": chrono::Utc::now(),
        });

        self.knowledge_base.update_topic("personality", &content).await?;
        Ok(())
    }

    pub fn set_update_interval(&mut self, interval: Duration) {
        self.update_interval = interval;
    }

    pub fn last_update_time(&self) -> std::time::Instant {
        self.last_update
    }

    pub async fn force_update(&mut self) -> Result<()> {
        info!("Forcing knowledge base update");
        self.update_all_knowledge().await
    }
}

// Add to main.rs to start the auto-updater
pub async fn start_knowledge_updater(pool: PgPool) {
    let mut updater = KnowledgeAutoUpdater::new(pool).expect("Failed to create knowledge updater");
    
    // Run the updater in a separate task
    tokio::spawn(async move {
        if let Err(e) = updater.start_auto_updates().await {
            error!("Knowledge updater error: {}", e);
        }
    });
} 