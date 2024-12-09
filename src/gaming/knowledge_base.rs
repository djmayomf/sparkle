use crate::games::{
    MarvelRivalsTrainer, POE2Trainer, MTGAPlayer, 
    ValorantTrainer, ApexTrainer, OverwatchTrainer,
    LeaguePlayer, FortnitePlayer
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use crate::resource_management::ResourceManager;

#[derive(Debug)]
pub struct GameKnowledge {
    games: DashMap<String, Arc<RwLock<dyn GameTrainer>>>,
    cache: Arc<DashMap<String, CachedResponse>>,
    resource_manager: Arc<ResourceManager>,
}

struct CachedResponse {
    response: String,
    expiry: std::time::Instant,
}

impl GameKnowledge {
    pub async fn new(resource_manager: Arc<ResourceManager>) -> Self {
        let mut knowledge = Self {
            games: DashMap::new(),
            cache: Arc::new(DashMap::new()),
            resource_manager,
        };

        // Initialize game trainers
        knowledge.init_trainers().await;

        knowledge
    }

    async fn init_trainers(&mut self) {
        // Initialize trainers with resource manager
        self.games.insert("marvel_rivals".to_string(), 
            Arc::new(RwLock::new(MarvelRivalsTrainer::new(self.resource_manager.clone()))));
        // Add other games...
    }

    pub async fn get_game_response(&self, context: &str) -> String {
        // Check cache first
        if let Some(cached) = self.check_cache(context) {
            return cached;
        }

        // Generate new response
        let response = self.generate_response(context).await;
        
        // Cache the response
        self.cache_response(context, &response);
        
        response
    }

    fn check_cache(&self, context: &str) -> Option<String> {
        if let Some(cached) = self.cache.get(context) {
            if cached.expiry > std::time::Instant::now() {
                return Some(cached.response.clone());
            }
            self.cache.remove(context);
        }
        None
    }

    fn cache_response(&self, context: &str, response: &str) {
        self.cache.insert(context.to_string(), CachedResponse {
            response: response.to_string(),
            expiry: std::time::Instant::now() + std::time::Duration::from_secs(300),
        });
    }
} 