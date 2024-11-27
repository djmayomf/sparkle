use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use super::design_spec::ModelDesignSpec;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelVersion {
    pub version: String,
    pub timestamp: DateTime<Utc>,
    pub design_spec: ModelDesignSpec,
    pub changes: Vec<ModelChange>,
    pub metrics: VersionMetrics,
    pub approved_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelChange {
    pub component: String,
    pub description: String,
    pub reason: ChangeReason,
    pub viewer_feedback_score: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChangeReason {
    ViewerFeedback,
    PerformanceOptimization,
    BrandConsistency,
    FeatureAddition,
    BugFix,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionMetrics {
    pub viewer_engagement: f32,
    pub performance_score: f32,
    pub stability_score: f32,
    pub feedback_positivity: f32,
}

pub struct ModelVersionControl {
    versions: HashMap<String, ModelVersion>,
    current_version: String,
    working_branch: Option<String>,
    auto_backup_enabled: bool,
}

impl ModelVersionControl {
    pub async fn new() -> Self {
        Self {
            versions: HashMap::new(),
            current_version: String::from("1.0.0"),
            working_branch: None,
            auto_backup_enabled: true,
        }
    }

    pub async fn create_working_branch(&mut self, branch_name: String) -> Result<(), Box<dyn std::error::Error>> {
        let current = self.versions.get(&self.current_version).cloned()
            .ok_or("Current version not found")?;
        
        self.working_branch = Some(branch_name.clone());
        self.versions.insert(branch_name, current);
        Ok(())
    }

    pub async fn commit_changes(&mut self, changes: Vec<ModelChange>) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(branch) = &self.working_branch {
            if let Some(version) = self.versions.get_mut(branch) {
                version.changes.extend(changes);
                version.timestamp = Utc::now();
            }
        }
        Ok(())
    }

    pub async fn merge_to_main(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(branch) = &self.working_branch {
            if let Some(working_version) = self.versions.get(branch).cloned() {
                // Increment version number
                let new_version = self.increment_version(&self.current_version);
                self.versions.insert(new_version.clone(), working_version);
                self.current_version = new_version;
                self.working_branch = None;
            }
        }
        Ok(())
    }

    fn increment_version(&self, version: &str) -> String {
        let mut parts: Vec<u32> = version.split('.')
            .map(|s| s.parse().unwrap_or(0))
            .collect();
        
        if parts.len() != 3 {
            parts = vec![1, 0, 0];
        } else {
            parts[2] += 1;
            if parts[2] >= 10 {
                parts[2] = 0;
                parts[1] += 1;
                if parts[1] >= 10 {
                    parts[1] = 0;
                    parts[0] += 1;
                }
            }
        }
        
        format!("{}.{}.{}", parts[0], parts[1], parts[2])
    }

    pub async fn auto_backup(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.auto_backup_enabled {
            // Implement backup logic
        }
        Ok(())
    }
} 