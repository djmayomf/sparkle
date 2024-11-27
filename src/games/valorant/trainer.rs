use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AgentRole {
    Duelist,
    Sentinel,
    Controller,
    Initiator,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Agent {
    pub name: String,
    pub role: AgentRole,
    pub difficulty: u8,
    pub abilities: Vec<String>,
    pub signature: String,
    pub ultimate: String,
    pub synergies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValorantSkillMetric {
    pub aim_accuracy: f32,
    pub ability_usage: f32,
    pub map_knowledge: f32,
    pub economy_management: f32,
    pub team_coordination: f32,
    pub crosshair_placement: f32,
    pub utility_impact: f32,
}

pub struct ValorantTrainer {
    agents: HashMap<String, Agent>,
    current_agent: Option<String>,
    skill_progress: ValorantSkillMetric,
    match_history: Vec<ValorantMatchResult>,
    training_schedule: Vec<TrainingTask>,
    rank_progress: RankProgress,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValorantMatchResult {
    pub timestamp: DateTime<Utc>,
    pub agent: String,
    pub map: String,
    pub rounds_won: u32,
    pub rounds_lost: u32,
    pub kda: (u32, u32, u32),
    pub performance_metrics: ValorantSkillMetric,
}

impl ValorantTrainer {
    pub fn new() -> Self {
        Self {
            agents: Self::init_agents(),
            current_agent: None,
            skill_progress: ValorantSkillMetric {
                aim_accuracy: 0.0,
                ability_usage: 0.0,
                map_knowledge: 0.0,
                economy_management: 0.0,
                team_coordination: 0.0,
                crosshair_placement: 0.0,
                utility_impact: 0.0,
            },
            match_history: Vec::new(),
            training_schedule: Vec::new(),
            rank_progress: RankProgress::default(),
        }
    }

    fn init_agents() -> HashMap<String, Agent> {
        let mut agents = HashMap::new();
        
        // Controllers
        agents.insert("brimstone".to_string(), Agent {
            name: "Brimstone".to_string(),
            role: AgentRole::Controller,
            difficulty: 3,
            abilities: vec!["Sky Smoke", "Stim Beacon", "Incendiary"].iter().map(String::from).collect(),
            signature: "Orbital Strike".to_string(),
            ultimate: "Orbital Strike".to_string(),
            synergies: vec!["Phoenix", "Raze", "Sage"].iter().map(String::from).collect(),
        });

        agents.insert("viper".to_string(), Agent {
            name: "Viper".to_string(),
            role: AgentRole::Controller,
            difficulty: 8,
            abilities: vec!["Poison Cloud", "Toxic Screen", "Snake Bite"].iter().map(String::from).collect(),
            signature: "Viper's Pit".to_string(),
            ultimate: "Viper's Pit".to_string(),
            synergies: vec!["Killjoy", "Cypher", "Sova"].iter().map(String::from).collect(),
        });

        // Duelists
        agents.insert("jett".to_string(), Agent {
            name: "Jett".to_string(),
            role: AgentRole::Duelist,
            difficulty: 7,
            abilities: vec!["Updraft", "Tailwind", "Cloudburst"].iter().map(String::from).collect(),
            signature: "Blade Storm".to_string(),
            ultimate: "Blade Storm".to_string(),
            synergies: vec!["Sage", "Omen", "Sova"].iter().map(String::from).collect(),
        });

        // Add all other agents...

        agents
    }

    pub async fn train_agent(&mut self, agent: &str) -> Result<String, String> {
        if let Some(agent_data) = self.agents.get(agent) {
            self.current_agent = Some(agent.to_string());
            self.generate_training_tasks(agent_data);
            
            Ok(format!("Starting Valorant training with {}! Focus on: {}", 
                agent_data.name,
                self.get_initial_focus_area(agent_data)
            ))
        } else {
            Err("Agent not found".to_string())
        }
    }

    fn get_initial_focus_area(&self, agent: &Agent) -> String {
        match agent.role {
            AgentRole::Sentinel => "Site anchoring and team support".to_string(),
            AgentRole::Duelist => "Entry fragging and site taking".to_string(),
            AgentRole::Controller => "Map control and vision denial".to_string(),
            AgentRole::Initiator => "Information gathering and team setup".to_string(),
        }
    }

    fn generate_training_tasks(&mut self, agent: &Agent) {
        self.training_schedule.clear();
        
        // Basic training tasks
        self.training_schedule.push(TrainingTask {
            focus_area: "Crosshair Placement".to_string(),
            agent: agent.name.clone(),
            completion_criteria: "Maintain head-level crosshair placement".to_string(),
            completed: false,
        });

        // Add role-specific tasks
        match agent.role {
            AgentRole::Sentinel => {
                self.add_sentinel_tasks(agent);
            },
            AgentRole::Duelist => {
                self.add_duelist_tasks(agent);
            },
            _ => {
                self.add_general_tasks(agent);
            }
        }
    }

    pub async fn update_match_stats(&mut self, result: ValorantMatchResult) {
        self.match_history.push(result.clone());
        self.update_skill_metrics(&result);
        self.adjust_training_schedule();
    }

    fn update_skill_metrics(&mut self, result: &ValorantMatchResult) {
        self.skill_progress = result.performance_metrics.clone();
        
        // Update training focus based on performance
        if self.skill_progress.aim_accuracy < 0.5 {
            self.add_aim_training_tasks();
        }
    }

    fn add_aim_training_tasks(&mut self) {
        self.training_schedule.push(TrainingTask {
            focus_area: "Aim Fundamentals".to_string(),
            agent: "Any".to_string(),
            completion_criteria: "Complete aim training routine with 80% accuracy".to_string(),
            completed: false,
        });
    }

    fn get_role_progression(&self, role: &AgentRole) -> Vec<&str> {
        match role {
            AgentRole::Controller => vec![
                "brimstone",  // Easiest
                "omen",
                "astra",
                "viper",      // Hardest
            ],
            AgentRole::Duelist => vec![
                "phoenix",    // Easiest
                "reyna",
                "raze",
                "yoru",
                "jett",       // Hardest
            ],
            AgentRole::Sentinel => vec![
                "sage",       // Easiest
                "cypher",
                "killjoy",
                "chamber",    // Hardest
            ],
            AgentRole::Initiator => vec![
                "sova",       // Easiest
                "breach",
                "skye",
                "kayo",       // Hardest
            ],
        }
    }
} 