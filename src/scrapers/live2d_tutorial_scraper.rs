use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::Client;
use tokio::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Live2DTutorialScraper {
    pub client: Client,
    pub playlists: Vec<PlaylistInfo>,
    pub tutorials: HashMap<String, TutorialContent>,
    pub techniques: HashMap<String, ModelingTechnique>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistInfo {
    pub id: String,
    pub name: String,
    pub category: TutorialCategory,
    pub videos: Vec<VideoInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TutorialCategory {
    Basics,
    Advanced,
    Professional,
    Rigging,
    Animation,
    Special,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoInfo {
    pub video_id: String,
    pub title: String,
    pub techniques: Vec<String>,
    pub key_points: Vec<String>,
    pub tools_used: Vec<String>,
    pub difficulty: TutorialDifficulty,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TutorialContent {
    pub title: String,
    pub steps: Vec<TutorialStep>,
    pub tools: Vec<ToolRequirement>,
    pub tips: Vec<String>,
    pub common_issues: Vec<IssueResolution>,
}

impl Live2DTutorialScraper {
    pub async fn new() -> Self {
        let mut scraper = Self {
            client: Client::new(),
            playlists: vec![
                PlaylistInfo {
                    id: "PLs1M2_VbQOf4C3EyfxRWgU_mop-bKL4cw".to_string(),
                    name: "Live2D Basics".to_string(),
                    category: TutorialCategory::Basics,
                    videos: Vec::new(),
                },
                PlaylistInfo {
                    id: "PLs1M2_VbQOf7fWVl33He5UAGvPqXfJ1-h".to_string(),
                    name: "Advanced Techniques".to_string(),
                    category: TutorialCategory::Advanced,
                    videos: Vec::new(),
                },
                PlaylistInfo {
                    id: "PLs1M2_VbQOf6bDLan8j-Tr7R-k9VDo3Pi".to_string(),
                    name: "Professional Rigging".to_string(),
                    category: TutorialCategory::Professional,
                    videos: Vec::new(),
                },
            ],
            tutorials: HashMap::new(),
            techniques: HashMap::new(),
            last_updated: chrono::Utc::now(),
        };

        scraper.initialize_tutorials().await;
        scraper
    }

    async fn initialize_tutorials(&mut self) {
        // Basic Techniques
        self.techniques.insert(
            "art_preparation".to_string(),
            ModelingTechnique {
                name: "Art Preparation".to_string(),
                steps: vec![
                    "Proper PSD layer organization".to_string(),
                    "Part separation guidelines".to_string(),
                    "Resolution requirements".to_string(),
                ],
                tools: vec!["Photoshop".to_string()],
                difficulty: TutorialDifficulty::Beginner,
            }
        );

        // Advanced Rigging
        self.techniques.insert(
            "advanced_deformation".to_string(),
            ModelingTechnique {
                name: "Advanced Deformation".to_string(),
                steps: vec![
                    "Parameter binding".to_string(),
                    "Weight painting".to_string(),
                    "Physics setup".to_string(),
                ],
                tools: vec!["Live2D Cubism".to_string()],
                difficulty: TutorialDifficulty::Advanced,
            }
        );

        // Professional Animation
        self.techniques.insert(
            "expression_rigging".to_string(),
            ModelingTechnique {
                name: "Expression Rigging".to_string(),
                steps: vec![
                    "Face deformation".to_string(),
                    "Expression parameters".to_string(),
                    "Blendshape creation".to_string(),
                ],
                tools: vec!["Live2D Cubism".to_string()],
                difficulty: TutorialDifficulty::Professional,
            }
        );
    }

    pub async fn get_tutorial(&self, technique: &str) -> Option<&TutorialContent> {
        self.tutorials.get(technique)
    }

    pub async fn get_technique_steps(&self, technique: &str) -> Option<&ModelingTechnique> {
        self.techniques.get(technique)
    }

    pub async fn generate_workflow(&self, model_type: ModelType) -> Vec<WorkflowStep> {
        match model_type {
            ModelType::Basic => self.generate_basic_workflow().await,
            ModelType::Advanced => self.generate_advanced_workflow().await,
            ModelType::Professional => self.generate_professional_workflow().await,
        }
    }

    async fn generate_basic_workflow(&self) -> Vec<WorkflowStep> {
        vec![
            WorkflowStep {
                name: "Art Preparation".to_string(),
                technique: "art_preparation".to_string(),
                estimated_time: 120,
                prerequisites: vec![],
            },
            WorkflowStep {
                name: "Basic Rigging".to_string(),
                technique: "basic_rigging".to_string(),
                estimated_time: 180,
                prerequisites: vec!["art_preparation".to_string()],
            },
        ]
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelingTechnique {
    pub name: String,
    pub steps: Vec<String>,
    pub tools: Vec<String>,
    pub difficulty: TutorialDifficulty,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TutorialStep {
    pub title: String,
    pub description: String,
    pub video_timestamp: Option<u32>,
    pub tools: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolRequirement {
    pub name: String,
    pub version: String,
    pub purpose: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueResolution {
    pub issue: String,
    pub solution: String,
    pub prevention_tips: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TutorialDifficulty {
    Beginner,
    Intermediate,
    Advanced,
    Professional,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ModelType {
    Basic,
    Advanced,
    Professional,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub name: String,
    pub technique: String,
    pub estimated_time: u32,  // in minutes
    pub prerequisites: Vec<String>,
} 