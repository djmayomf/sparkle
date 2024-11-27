use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::broadcast;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelVersion {
    pub version: String,
    pub release_date: DateTime<Utc>,
    pub features: Vec<String>,
    pub animations: HashMap<String, String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelUpdate {
    pub from_version: String,
    pub to_version: String,
    pub changes: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

pub struct ModelManager {
    current_version: ModelVersion,
    available_versions: HashMap<String, ModelVersion>,
    update_history: Vec<ModelUpdate>,
    event_sender: broadcast::Sender<ModelUpdate>,
}

impl ModelManager {
    pub fn new(initial_version: ModelVersion) -> (Self, broadcast::Receiver<ModelUpdate>) {
        let (tx, rx) = broadcast::channel(100);
        let mut versions = HashMap::new();
        versions.insert(initial_version.version.clone(), initial_version.clone());

        (Self {
            current_version: initial_version,
            available_versions: versions,
            update_history: Vec::new(),
            event_sender: tx,
        }, rx)
    }

    pub async fn update_model(&mut self, new_version: String) -> Result<(), String> {
        if let Some(version) = self.available_versions.get(&new_version) {
            if !version.is_active {
                return Err("Version is not ready for activation".to_string());
            }

            let update = ModelUpdate {
                from_version: self.current_version.version.clone(),
                to_version: new_version.clone(),
                changes: version.features.clone(),
                timestamp: Utc::now(),
            };

            self.current_version = version.clone();
            self.update_history.push(update.clone());
            self.broadcast_update(update).await;
            Ok(())
        } else {
            Err("Version not found".to_string())
        }
    }

    pub async fn add_version(&mut self, version: ModelVersion) {
        self.available_versions.insert(version.version.clone(), version);
    }

    async fn broadcast_update(&self, update: ModelUpdate) {
        let _ = self.event_sender.send(update);
    }

    pub fn get_current_version(&self) -> &ModelVersion {
        &self.current_version
    }

    pub fn list_available_versions(&self) -> Vec<&ModelVersion> {
        self.available_versions.values().collect()
    }

    pub fn get_update_history(&self) -> &[ModelUpdate] {
        &self.update_history
    }
} 