use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct VTubeTutorialAnalyzer {
    pub playlists: HashMap<String, PlaylistContent>,
    pub techniques: HashMap<String, ModelingTechnique>,
    pub workflows: Vec<WorkflowGuide>,
    pub best_practices: Vec<BestPractice>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistContent {
    pub playlist_id: String,
    pub focus_area: ModelingFocus,
    pub key_lessons: Vec<LessonPoint>,
    pub techniques_covered: Vec<String>,
    pub skill_level: SkillLevel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelingTechnique {
    pub name: String,
    pub software: String,
    pub workflow_steps: Vec<WorkflowStep>,
    pub common_issues: Vec<IssueGuide>,
    pub best_practices: Vec<String>,
}

impl VTubeTutorialAnalyzer {
    pub async fn new() -> Self {
        let mut analyzer = Self {
            playlists: HashMap::new(),
            techniques: HashMap::new(),
            workflows: Vec::new(),
            best_practices: Vec::new(),
            last_updated: Utc::now(),
        };

        analyzer.initialize_knowledge_base().await;
        analyzer
    }

    async fn initialize_knowledge_base(&mut self) {
        // Live2D Basics Playlist
        self.playlists.insert(
            "PLKiJyhnUIen1XDHRyAg08Y2ohET23mW6g".to_string(),
            PlaylistContent {
                playlist_id: "PLKiJyhnUIen1XDHRyAg08Y2ohET23mW6g".to_string(),
                focus_area: ModelingFocus::Live2DBasics,
                key_lessons: vec![
                    LessonPoint {
                        topic: "Art Preparation".to_string(),
                        key_points: vec![
                            "Layer organization".to_string(),
                            "Part separation".to_string(),
                            "Resolution guidelines".to_string(),
                        ],
                        importance: Importance::Critical,
                    }
                ],
                techniques_covered: vec![
                    "PSD Setup".to_string(),
                    "Basic Deformation".to_string(),
                ],
                skill_level: SkillLevel::Beginner,
            }
        );

        // Advanced Rigging Techniques
        self.techniques.insert(
            "advanced_rigging".to_string(),
            ModelingTechnique {
                name: "Advanced Rigging".to_string(),
                software: "Live2D Cubism".to_string(),
                workflow_steps: vec![
                    WorkflowStep {
                        name: "Parameter Setup".to_string(),
                        description: "Create core deformation parameters".to_string(),
                        tools_needed: vec!["Cubism Editor".to_string()],
                        duration_estimate: 120,
                    }
                ],
                common_issues: vec![
                    IssueGuide {
                        problem: "Deformation artifacts".to_string(),
                        solution: "Adjust weight painting".to_string(),
                        prevention: "Proper mesh setup".to_string(),
                    }
                ],
                best_practices: vec![
                    "Use parameter groups".to_string(),
                    "Test all deformation ranges".to_string(),
                ],
            }
        );
    }

    pub async fn get_workflow(&self, focus: ModelingFocus) -> Option<&WorkflowGuide> {
        self.workflows.iter().find(|w| w.focus == focus)
    }

    pub async fn get_technique_guide(&self, technique: &str) -> Option<&ModelingTechnique> {
        self.techniques.get(technique)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ModelingFocus {
    Live2DBasics,
    AdvancedRigging,
    ExpressionSetup,
    PhysicsSystem,
    TextureWork,
    MotionSetup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LessonPoint {
    pub topic: String,
    pub key_points: Vec<String>,
    pub importance: Importance,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowGuide {
    pub focus: ModelingFocus,
    pub steps: Vec<WorkflowStep>,
    pub requirements: Vec<String>,
    pub estimated_time: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub name: String,
    pub description: String,
    pub tools_needed: Vec<String>,
    pub duration_estimate: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueGuide {
    pub problem: String,
    pub solution: String,
    pub prevention: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BestPractice {
    pub category: String,
    pub practice: String,
    pub reason: String,
    pub examples: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    Professional,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Importance {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TutorialMetrics {
    pub completion_time: u32,
    pub difficulty_rating: f32,
    pub success_rate: f32,
    pub common_issues: Vec<String>,
} 