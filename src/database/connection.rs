use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use crate::error::{AppError, Result};
use serde_json;

#[derive(Clone)]
pub struct DatabaseConnection {
    pool: PgPool,
}

impl DatabaseConnection {
    pub async fn new() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| AppError::Config("DATABASE_URL must be set".to_string()))?;

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&database_url)
            .await
            .map_err(AppError::Database)?;

        Ok(Self { pool })
    }

    pub fn get_pool(&self) -> PgPool {
        self.pool.clone()
    }
}

pub struct KnowledgeBaseManager {
    pool: PgPool,
}

impl KnowledgeBaseManager {
    pub async fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn add_topic(&self, topic: &str, content: &serde_json::Value) -> Result<()> {
        sqlx::query!(
            "INSERT INTO knowledge_base (topic, content) VALUES ($1, $2)",
            topic,
            content
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(())
    }

    pub async fn update_topic(&self, topic: &str, content: &serde_json::Value) -> Result<()> {
        sqlx::query!(
            "UPDATE knowledge_base SET content = $2, last_updated = CURRENT_TIMESTAMP WHERE topic = $1",
            topic,
            content
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(())
    }

    pub async fn get_topic(&self, topic: &str) -> Result<serde_json::Value> {
        let record = sqlx::query!(
            "SELECT content FROM knowledge_base WHERE topic = $1",
            topic
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;
        
        Ok(record.content)
    }
}
