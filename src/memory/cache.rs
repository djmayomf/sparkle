use crate::database::connection::DatabaseConnection;
use std::sync::Arc;

pub struct MemoryCache {
    db: Arc<DatabaseConnection>,
}

impl MemoryCache {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
} 