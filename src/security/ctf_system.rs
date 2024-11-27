use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::time::Duration;
use super::ctf_scraper::CTFScraper;

#[derive(Debug, Serialize, Deserialize)]
pub struct CTFSystem {
    pub current_challenge: Option<Challenge>,
    pub completed_challenges: Vec<String>,
    pub skill_levels: HashMap<CTFCategory, f32>,
    pub learning_progress: HashMap<String, f32>,
    pub active_tools: Vec<SecurityTool>,
    knowledge_base: CTFScraper,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Challenge {
    pub name: String,
    pub category: CTFCategory,
    pub difficulty: u8,
    pub points: u32,
    pub description: String,
    pub hints: Vec<String>,
    pub solution_steps: Vec<SolutionStep>,
    pub time_limit: Option<Duration>,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum CTFCategory {
    WebExploitation,
    Cryptography,
    ReverseEngineering,
    Forensics,
    BinaryExploitation,
    NetworkSecurity,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolutionStep {
    pub tool: String,
    pub command: String,
    pub expected_output: String,
    pub explanation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityTool {
    pub name: String,
    pub purpose: String,
    pub command_syntax: String,
    pub skill_required: f32,
}

impl CTFSystem {
    pub async fn new() -> Self {
        Self {
            current_challenge: None,
            completed_challenges: Vec::new(),
            skill_levels: Self::initialize_skills(),
            learning_progress: HashMap::new(),
            active_tools: Vec::new(),
            knowledge_base: CTFScraper::new().await.unwrap(),
        }
    }

    fn initialize_skills() -> HashMap<CTFCategory, f32> {
        let mut skills = HashMap::new();
        skills.insert(CTFCategory::WebExploitation, 0.0);
        skills.insert(CTFCategory::Cryptography, 0.0);
        skills.insert(CTFCategory::ReverseEngineering, 0.0);
        skills.insert(CTFCategory::Forensics, 0.0);
        skills.insert(CTFCategory::BinaryExploitation, 0.0);
        skills.insert(CTFCategory::NetworkSecurity, 0.0);
        skills
    }

    pub async fn start_challenge(&mut self, challenge: Challenge) -> Result<(), String> {
        // Set up the challenge environment
        self.current_challenge = Some(challenge);
        self.prepare_tools().await?;
        Ok(())
    }

    pub async fn get_challenge_help(&self, category: &str) -> Result<Vec<String>, String> {
        if let Some(hints) = self.knowledge_base.get_challenge_hints(category, "general").await {
            Ok(hints)
        } else {
            Err("No hints available for this category".to_string())
        }
    }

    pub async fn prepare_tools(&mut self) -> Result<(), String> {
        if let Some(challenge) = &self.current_challenge {
            // Get tool recommendations from knowledge base
            for step in &challenge.solution_steps {
                if let Some(tool_info) = self.knowledge_base.get_tool_usage(&step.tool).await {
                    // Load tool with proper configuration
                    self.load_tool_with_config(&tool_info).await?;
                }
            }
        }
        Ok(())
    }

    async fn load_tool_with_config(&mut self, tool_info: &CTFTool) -> Result<(), String> {
        let tool = SecurityTool {
            name: tool_info.name.clone(),
            purpose: tool_info.purpose.clone(),
            command_syntax: tool_info.usage_examples.join("\n"),
            skill_required: 0.5, // Adjust based on tool complexity
        };
        self.active_tools.push(tool);
        Ok(())
    }

    pub async fn solve_step(&mut self, step_index: usize) -> Result<bool, String> {
        if let Some(challenge) = &self.current_challenge {
            if let Some(step) = challenge.solution_steps.get(step_index) {
                // Execute the solution step
                // Update skill levels and learning progress
                self.update_skills(&challenge.category).await;
                return Ok(true);
            }
        }
        Ok(false)
    }

    async fn update_skills(&mut self, category: &CTFCategory) {
        if let Some(skill_level) = self.skill_levels.get_mut(category) {
            *skill_level += 0.1;
            *skill_level = skill_level.min(1.0);
        }
    }
} 