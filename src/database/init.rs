use sqlx::postgres::PgPool;
use crate::error::Result;

pub async fn initialize_database(pool: &PgPool) -> Result<()> {
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| AppError::Database(e))?;

    // Initialize all required tables
    initialize_knowledge_base(pool).await?;
    initialize_autonomy_data(pool).await?;
    initialize_emotional_data(pool).await?;
    initialize_learning_data(pool).await?;

    Ok(())
}

async fn initialize_knowledge_base(pool: &PgPool) -> Result<()> {
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

async fn initialize_autonomy_data(pool: &PgPool) -> Result<()> {
    let default_configs = vec![
        ("personality", serde_json::json!({
            "core_traits": {
                "openness": 0.8,
                "conscientiousness": 0.85,
                "extraversion": 0.7,
                "agreeableness": 0.9,
                "stability": 0.75
            },
            "behavioral_tendencies": [],
            "social_preferences": {
                "interaction_style": "friendly",
                "communication_preference": "direct",
                "group_dynamics": "supportive"
            }
        })),
        ("emotional_core", serde_json::json!({
            "base_mood": "positive",
            "emotional_range": [0.3, 0.9],
            "stability_factor": 0.8,
            "empathy_level": 0.85
        })),
        ("learning_config", serde_json::json!({
            "learning_rate": 0.1,
            "adaptation_speed": 0.7,
            "improvement_focus": ["social_skills", "knowledge_depth", "response_timing"]
        })),
    ];

    for (config_type, content) in default_configs {
        sqlx::query!(
            r#"
            INSERT INTO autonomy_config (config_type, content)
            VALUES ($1, $2)
            ON CONFLICT (config_type) DO UPDATE
            SET content = $2
            "#,
            config_type,
            content
        )
        .execute(pool)
        .await
        .map_err(AppError::Database)?;
    }

    Ok(())
}

async fn initialize_emotional_data(pool: &PgPool) -> Result<()> {
    let emotional_patterns = vec![
        ("greeting", serde_json::json!({
            "base_emotion": "joy",
            "intensity_range": [0.5, 0.8],
            "expression_style": "warm",
            "context_adaptations": ["time_of_day", "user_familiarity", "stream_energy"]
        })),
        ("learning", serde_json::json!({
            "base_emotion": "interest",
            "intensity_range": [0.6, 0.9],
            "expression_style": "curious",
            "context_adaptations": ["topic_complexity", "user_engagement", "learning_progress"]
        })),
    ];

    for (pattern_type, content) in emotional_patterns {
        sqlx::query!(
            r#"
            INSERT INTO emotional_patterns (pattern_type, content)
            VALUES ($1, $2)
            ON CONFLICT (pattern_type) DO UPDATE
            SET content = $2
            "#,
            pattern_type,
            content
        )
        .execute(pool)
        .await
        .map_err(AppError::Database)?;
    }

    Ok(())
}

async fn initialize_learning_data(pool: &PgPool) -> Result<()> {
    let learning_configs = vec![
        ("skill_tracking", serde_json::json!({
            "tracked_skills": {},
            "learning_thresholds": {
                "beginner": 0.3,
                "intermediate": 0.6,
                "advanced": 0.8,
                "expert": 0.95
            },
            "improvement_metrics": ["accuracy", "speed", "complexity", "creativity"]
        })),
        ("knowledge_domains", serde_json::json!({
            "anime": { "proficiency": 0.9, "last_updated": null },
            "gaming": { "proficiency": 0.85, "last_updated": null },
            "technology": { "proficiency": 0.8, "last_updated": null },
            "social_dynamics": { "proficiency": 0.75, "last_updated": null }
        }))
    ];

    for (config_type, content) in learning_configs {
        sqlx::query!(
            r#"
            INSERT INTO learning_config (config_type, content)
            VALUES ($1, $2)
            ON CONFLICT (config_type) DO UPDATE
            SET content = $2
            "#,
            config_type,
            content
        )
        .execute(pool)
        .await
        .map_err(AppError::Database)?;
    }

    Ok(())
} 