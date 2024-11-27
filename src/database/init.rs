use sqlx::postgres::PgPool;
use crate::error::Result;

pub async fn initialize_database(pool: &PgPool) -> Result<()> {
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| AppError::Database(e))?;

    // Initialize default knowledge base entries if needed
    let count = sqlx::query!(
        "SELECT COUNT(*) as count FROM knowledge_base"
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?
    .count
    .unwrap_or(0);

    if count == 0 {
        initialize_default_knowledge(pool).await?;
    }

    Ok(())
}

async fn initialize_default_knowledge(pool: &PgPool) -> Result<()> {
    let default_topics = vec![
        ("security", serde_json::json!({"type": "base_knowledge", "content": {}})),
        ("personality", serde_json::json!({"type": "base_traits", "content": {}})),
        ("games", serde_json::json!({"type": "supported_games", "content": {}})),
    ];

    for (topic, content) in default_topics {
        sqlx::query!(
            "INSERT INTO knowledge_base (topic, content) VALUES ($1, $2)",
            topic,
            content
        )
        .execute(pool)
        .await
        .map_err(AppError::Database)?;
    }

    Ok(())
} 