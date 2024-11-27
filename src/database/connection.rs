use sqlx::postgres::{PgPool, PgPoolOptions};
use crate::error::{AppError, Result};
use std::time::Duration;

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

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
} 