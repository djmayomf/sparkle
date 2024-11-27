use sqlx::postgres::PgPool;
use sqlx::migrate::MigrateError;
use sqlx::types::chrono::{Utc, DateTime};
use crate::error::{AppError, Result};
use std::path::Path;

pub async fn initialize_database(pool: &PgPool) -> Result<()> {
    // Check if migrations directory exists
    if !Path::new("./migrations").exists() {
        return Err(AppError::Config("Migrations directory not found".to_string()));
    }

    // Run migrations with proper error conversion
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e: MigrateError| AppError::Database(e.into()))?;

    // Initialize default knowledge base entries if needed
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM knowledge_base")
        .fetch_one(pool)
        .await
        .map_err(AppError::Database)?;

    if count == 0 {
        initialize_default_knowledge(pool).await?;
    }

    Ok(())
}

async fn initialize_default_knowledge(pool: &PgPool) -> Result<()> {
    let current_time = Utc::now().to_rfc3339();
    
    let default_topics = vec![
        ("security", serde_json::json!({
            "type": "base_knowledge",
            "content": {
                "rules": [],
                "policies": [],
                "last_updated": current_time
            }
        })),
        ("personality", serde_json::json!({
            "type": "base_traits",
            "content": {
                "traits": ["friendly", "helpful", "professional"],
                "voice_style": "natural",
                "last_updated": current_time
            }
        })),
        ("games", serde_json::json!({
            "type": "supported_games",
            "content": {
                "games": [],
                "settings": {},
                "last_updated": current_time
            }
        })),
    ];

    for (topic, content) in default_topics {
        sqlx::query(
            "INSERT INTO knowledge_base (topic, content) VALUES ($1, $2)"
        )
        .bind(topic)
        .bind(content)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;
    }

    Ok(())
} 