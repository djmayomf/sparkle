use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelCreationAnalyzer {
    pub playlist_id: String,
    pub tutorials: Vec<TutorialContent>,
    pub techniques: HashMap<String, ModelingTechnique>,
    pub workflow_steps: Vec<WorkflowStep>,
    pub best_practices: Vec<BestPractice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TutorialContent {
    pub video_id: String,
    pub title: String,
    pub focus_area: FocusArea,
    pub key_points: Vec<String>,
    pub tools_used: Vec<String>,
    pub timestamps: HashMap<String, f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelingTechnique {
    pub name: String,
    pub description: String,
    pub difficulty: Difficulty,
    pub prerequisites: Vec<String>,
    pub steps: Vec<TechniqueStep>,
}

impl ModelCreationAnalyzer {
    pub async fn new() -> Self {
        let mut analyzer = Self {
            playlist_id: "PLmSH_v3LAuebokNwli15vM_yU9ixcQiDu".to_string(),
            tutorials: Vec::new(),
            techniques: HashMap::new(),
            workflow_steps: Vec::new(),
            best_practices: Vec::new(),
        };

        analyzer.initialize_knowledge_base().await;
        analyzer
    }

    async fn initialize_knowledge_base(&mut self) {
        // Initialize key tutorials
        self.tutorials = vec![
            TutorialContent {
                video_id: "video1".to_string(),
                title: "Model Creation Basics".to_string(),
                focus_area: FocusArea::ArtPreparation,
                key_points: vec![
                    "Layer organization".to_string(),
                    "Part separation".to_string(),
                    "Resolution requirements".to_string(),
                ],
                tools_used: vec![
                    "Photoshop".to_string(),
                    "Live2D Cubism".to_string(),
                ],
                timestamps: HashMap::new(),
            }
        ];

        // Initialize modeling techniques
        self.techniques.insert(
            "art_prep".to_string(),
            ModelingTechnique {
                name: "Art Preparation".to_string(),
                description: "Preparing artwork for Live2D rigging".to_string(),
                difficulty: Difficulty::Beginner,
                prerequisites: vec![],
                steps: vec![
                    TechniqueStep {
                        name: "Layer Organization".to_string(),
                        description: "Organize layers by body parts".to_string(),
                        duration: 30,
                        tools: vec!["Photoshop".to_string()],
                    }
                ],
            }
        );

        // Initialize workflow steps
        self.workflow_steps = vec![
            WorkflowStep {
                name: "Initial Setup".to_string(),
                order: 1,
                description: "Setting up the project and tools".to_string(),
                estimated_time: 60,
                required_tools: vec![
                    "Photoshop".to_string(),
                    "Live2D Cubism".to_string(),
                ],
            }
        ];

        // Initialize best practices
        self.best_practices = vec![
            BestPractice {
                category: "File Organization".to_string(),
                practice: "Use clear layer naming".to_string(),
                reason: "Improves workflow efficiency".to_string(),
                impact_level: ImpactLevel::High,
            }
        ];
    }

    pub async fn get_workflow(&self, focus: FocusArea) -> Vec<WorkflowStep> {
        self.workflow_steps.iter()
            .filter(|step| step.focus_areas.contains(&focus))
            .cloned()
            .collect()
    }

    pub async fn get_technique_guide(&self, technique: &str) -> Option<&ModelingTechnique> {
        self.techniques.get(technique)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum FocusArea {
    ArtPreparation,
    Rigging,
    Deformation,
    Physics,
    Expressions,
    Animation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechniqueStep {
    pub name: String,
    pub description: String,
    pub duration: u32,  // minutes
    pub tools: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub name: String,
    pub order: u32,
    pub description: String,
    pub estimated_time: u32,  // minutes
    pub required_tools: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BestPractice {
    pub category: String,
    pub practice: String,
    pub reason: String,
    pub impact_level: ImpactLevel,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ImpactLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LearningPath {
    pub steps: Vec<LearningStep>,
    pub total_duration: u32,
    pub prerequisites: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LearningStep {
    pub name: String,
    pub resources: Vec<String>,
    pub practice_exercises: Vec<String>,
    pub completion_criteria: Vec<String>,
} 