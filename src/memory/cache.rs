use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use tokio::task::spawn_blocking;
use sqlx::postgres::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub content: String,
    pub context: String,
    pub timestamp: DateTime<Utc>,
    pub relevance_score: f32,
    pub access_count: u32,
}

pub struct MemoryCache {
    short_term: HashMap<String, MemoryEntry>,
    long_term: HashMap<String, MemoryEntry>,
    facts_cache: HashMap<String, MemoryEntry>,
}

impl MemoryCache {
    pub fn new() -> Self {
        Self {
            short_term: HashMap::new(),
            long_term: HashMap::new(),
            facts_cache: HashMap::new(),
        }
    }

    pub fn add_memory(&mut self, key: String, content: String, context: String) {
        let entry = MemoryEntry {
            content,
            context,
            timestamp: Utc::now(),
            relevance_score: 1.0,
            access_count: 1,
        };

        self.short_term.insert(key.clone(), entry.clone());
        
        tokio::spawn(async move {
            if let Err(e) = Self::persist_memory(&key, &entry).await {
                eprintln!("Failed to persist memory: {}", e);
            }
        });
    }

    pub fn get_memory(&mut self, key: &str) -> Option<&MemoryEntry> {
        if let Some(entry) = self.short_term.get(key) {
            return Some(entry);
        }

        if let Some(entry) = self.long_term.get(key) {
            if entry.access_count > 10 {
                let mut entry = entry.clone();
                entry.access_count += 1;
                self.short_term.insert(key.to_string(), entry);
                self.long_term.remove(key);
            }
            return Some(entry);
        }

        None
    }

    async fn persist_memory(key: &str, entry: &MemoryEntry) -> Result<(), Error> {
        let pool = get_db_pool().await?;
        
        sqlx::query!(
            "INSERT INTO memory_store (memory_type, content, context, relevance_score) 
             VALUES ($1, $2, $3, $4)",
            "short_term",
            serde_json::to_value(entry)?,
            entry.context,
            entry.relevance_score
        )
        .execute(&pool)
        .await?;

        Ok(())
    }

    pub fn consolidate_memories(&mut self) {
        let now = Utc::now();
        
        self.short_term.retain(|key, entry| {
            let age = now - entry.timestamp;
            if age > Duration::hours(24) {
                self.long_term.insert(key.clone(), entry.clone());
                false
            } else {
                true
            }
        });
    }

    pub fn cache_fact(&mut self, category: String, fact: String) {
        let entry = MemoryEntry {
            content: fact,
            context: category,
            timestamp: Utc::now(),
            relevance_score: 1.0,
            access_count: 0,
        };
        self.facts_cache.insert(entry.content.clone(), entry);
    }
} 