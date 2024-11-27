use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct SoftwareRequirement {
    pub name: String,
    pub purpose: String,
    pub category: SoftwareCategory,
    pub download_url: Option<String>,
    pub version: Option<String>,
    pub is_installed: bool,
    pub dependencies: Vec<SoftwareDependency>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SoftwareCategory {
    ModelCreation,
    TechDemo,
    StreamingTools,
    Development,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoftwareDependency {
    pub name: String,
    pub minimum_version: Option<String>,
    pub is_installed: bool,
}

pub struct SoftwareTracker {
    db: Database, // Your database connection
}

impl SoftwareTracker {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn check_model_creation_requirements(&self) -> Result<Vec<SoftwareRequirement>, Box<dyn Error>> {
        let requirements = vec![
            SoftwareRequirement {
                name: "VTube Studio".to_string(),
                purpose: "Base VTuber model creation and live2D integration".to_string(),
                category: SoftwareCategory::ModelCreation,
                download_url: Some("https://denchisoft.com/".to_string()),
                version: None,
                is_installed: false,
                dependencies: vec![],
            },
            SoftwareRequirement {
                name: "Live2D Cubism".to_string(),
                purpose: "Model rigging and animation".to_string(),
                category: SoftwareCategory::ModelCreation,
                download_url: Some("https://www.live2d.com/en/download/cubism/".to_string()),
                version: None,
                is_installed: false,
                dependencies: vec![],
            },
            // Add more model creation software as needed
        ];

        self.db.update_software_requirements(&requirements).await?;
        Ok(requirements)
    }

    pub async fn check_tech_demo_requirements(&self) -> Result<Vec<SoftwareRequirement>, Box<dyn Error>> {
        let requirements = vec![
            SoftwareRequirement {
                name: "OBS Studio".to_string(),
                purpose: "Streaming and demo recording".to_string(),
                category: SoftwareCategory::TechDemo,
                download_url: Some("https://obsproject.com/".to_string()),
                version: None,
                is_installed: false,
                dependencies: vec![],
            },
            SoftwareRequirement {
                name: "Virtual Audio Cable".to_string(),
                purpose: "Audio routing for demos".to_string(),
                category: SoftwareCategory::TechDemo,
                download_url: Some("https://vb-audio.com/Cable/".to_string()),
                version: None,
                is_installed: false,
                dependencies: vec![],
            },
            // Add more tech demo software as needed
        ];

        self.db.update_software_requirements(&requirements).await?;
        Ok(requirements)
    }

    pub async fn notify_missing_software(&self) -> Result<(), Box<dyn Error>> {
        let missing = self.db.get_uninstalled_software().await?;
        if !missing.is_empty() {
            // Implement notification system here
            println!("Missing required software:");
            for software in missing {
                println!("- {}: {} ({})", software.name, software.purpose, software.download_url.unwrap_or_default());
            }
        }
        Ok(())
    }

    pub async fn mark_as_installed(&self, software_name: &str) -> Result<(), Box<dyn Error>> {
        self.db.mark_software_installed(software_name, true).await?;
        Ok(())
    }
} 