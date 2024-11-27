use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum LegendRole {
    Assault,
    Recon,
    Support,
    Controller,
    Skirmisher,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Legend {
    pub name: String,
    pub role: LegendRole,
    pub difficulty: u8, // 1-10
    pub abilities: Vec<String>,
    pub tactical: String,
    pub ultimate: String,
    pub passive: String,
    pub synergies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApexSkillMetric {
    pub aim_accuracy: f32,
    pub movement: f32,
    pub positioning: f32,
    pub ability_usage: f32,
    pub team_coordination: f32,
    pub looting_speed: f32,
    pub map_awareness: f32,
}

pub struct ApexTrainer {
    legends: HashMap<String, Legend>,
    current_legend: Option<String>,
    skill_progress: ApexSkillMetric,
    match_history: Vec<ApexMatchResult>,
    training_schedule: Vec<TrainingTask>,
    rank_progress: RankProgress,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApexMatchResult {
    pub timestamp: DateTime<Utc>,
    pub legend: String,
    pub map: String,
    pub placement: u32,
    pub kills: u32,
    pub damage: u32,
    pub performance_metrics: ApexSkillMetric,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingTask {
    pub focus_area: String,
    pub legend: String,
    pub completion_criteria: String,
    pub completed: bool,
}

impl ApexTrainer {
    pub fn new() -> Self {
        Self {
            legends: Self::init_legends(),
            current_legend: None,
            skill_progress: ApexSkillMetric {
                aim_accuracy: 0.0,
                movement: 0.0,
                positioning: 0.0,
                ability_usage: 0.0,
                team_coordination: 0.0,
                looting_speed: 0.0,
                map_awareness: 0.0,
            },
            match_history: Vec::new(),
            training_schedule: Vec::new(),
            rank_progress: RankProgress::default(),
        }
    }

    fn init_legends() -> HashMap<String, Legend> {
        let mut legends = HashMap::new();
        
        // Support Legends
        legends.insert("lifeline".to_string(), Legend {
            name: "Lifeline".to_string(),
            role: LegendRole::Support,
            difficulty: 3,
            abilities: vec!["Combat Revive".to_string()],
            tactical: "D.O.C. Heal Drone".to_string(),
            ultimate: "Care Package".to_string(),
            passive: "Combat Medic".to_string(),
            synergies: vec!["Gibraltar", "Octane", "Pathfinder"].iter().map(String::from).collect(),
        });

        // Recon Legends
        legends.insert("bloodhound".to_string(), Legend {
            name: "Bloodhound".to_string(),
            role: LegendRole::Recon,
            difficulty: 5,
            abilities: vec!["Eye of the Allfather".to_string()],
            tactical: "Eye of the Allfather".to_string(),
            ultimate: "Beast of the Hunt".to_string(),
            passive: "Tracker".to_string(),
            synergies: vec!["Gibraltar", "Bangalore", "Caustic"].iter().map(String::from).collect(),
        });

        // Assault Legends
        legends.insert("wraith".to_string(), Legend {
            name: "Wraith".to_string(),
            role: LegendRole::Assault,
            difficulty: 8,
            abilities: vec!["Into the Void".to_string()],
            tactical: "Into the Void".to_string(),
            ultimate: "Dimensional Rift".to_string(),
            passive: "Voices from the Void".to_string(),
            synergies: vec!["Pathfinder", "Octane", "Lifeline"].iter().map(String::from).collect(),
        });

        // Add all other legends...

        legends
    }

    fn get_role_progression(&self, role: &LegendRole) -> Vec<&str> {
        match role {
            LegendRole::Support => vec![
                "lifeline",    // Easiest
                "loba",
                "gibraltar",   // Hardest
            ],
            LegendRole::Recon => vec![
                "bloodhound",  // Easiest
                "crypto",
                "seer",
                "valkyrie",    // Hardest
            ],
            LegendRole::Assault => vec![
                "bangalore",   // Easiest
                "octane",
                "horizon",
                "wraith",      // Hardest
            ],
            LegendRole::Controller => vec![
                "caustic",     // Easiest
                "wattson",
                "rampart",     // Hardest
            ],
            LegendRole::Skirmisher => vec![
                "pathfinder",  // Easiest
                "mirage",
                "revenant",    // Hardest
            ],
        }
    }

    pub async fn train_legend(&mut self, legend: &str) -> Result<String, String> {
        if let Some(legend_data) = self.legends.get(legend) {
            self.current_legend = Some(legend.to_string());
            self.generate_training_tasks(legend_data);
            
            Ok(format!("Starting Apex training with {}! Focus on: {}", 
                legend_data.name,
                self.get_initial_focus_area(legend_data)
            ))
        } else {
            Err("Legend not found".to_string())
        }
    }

    fn get_initial_focus_area(&self, legend: &Legend) -> String {
        match legend.role {
            LegendRole::Support => "Team positioning and healing efficiency".to_string(),
            LegendRole::Assault => "Aggressive positioning and aim training".to_string(),
            LegendRole::Recon => "Map awareness and information gathering".to_string(),
            LegendRole::Controller => "Area control and team coordination".to_string(),
            LegendRole::Skirmisher => "Movement and engagement timing".to_string(),
        }
    }

    fn generate_training_tasks(&mut self, legend: &Legend) {
        self.training_schedule.clear();
        
        // Basic movement training
        self.training_schedule.push(TrainingTask {
            focus_area: "Movement Fundamentals".to_string(),
            legend: legend.name.clone(),
            completion_criteria: "Master sliding, wall jumping, and basic movement tech".to_string(),
            completed: false,
        });

        // Add role-specific tasks
        match legend.role {
            LegendRole::Support => {
                self.add_support_tasks(legend);
            },
            LegendRole::Assault => {
                self.add_assault_tasks(legend);
            },
            _ => {
                self.add_general_tasks(legend);
            }
        }
    }

    fn add_support_tasks(&mut self, legend: &Legend) {
        self.training_schedule.extend(vec![
            TrainingTask {
                focus_area: "Team Healing".to_string(),
                legend: legend.name.clone(),
                completion_criteria: "Maintain team health above 75% during engagements".to_string(),
                completed: false,
            },
            TrainingTask {
                focus_area: "Safe Revives".to_string(),
                legend: legend.name.clone(),
                completion_criteria: "Successfully revive teammates under pressure".to_string(),
                completed: false,
            },
        ]);
    }

    fn add_assault_tasks(&mut self, legend: &Legend) {
        self.training_schedule.extend(vec![
            TrainingTask {
                focus_area: "Aim Training".to_string(),
                legend: legend.name.clone(),
                completion_criteria: "Achieve 60% accuracy in firing range".to_string(),
                completed: false,
            },
            TrainingTask {
                focus_area: "Aggressive Positioning".to_string(),
                legend: legend.name.clone(),
                completion_criteria: "Successfully push and secure kills".to_string(),
                completed: false,
            },
        ]);
    }

    pub async fn update_match_stats(&mut self, result: ApexMatchResult) {
        self.match_history.push(result.clone());
        self.update_skill_metrics(&result);
        self.update_rank(&result);
    }

    fn update_skill_metrics(&mut self, result: &ApexMatchResult) {
        // Update based on match performance
        self.skill_progress = result.performance_metrics.clone();
        self.adjust_training_schedule();
    }

    fn adjust_training_schedule(&mut self) {
        // Remove completed tasks
        self.training_schedule.retain(|task| !task.completed);

        // Add advanced tasks based on progress
        if self.training_schedule.is_empty() {
            self.add_advanced_tasks();
        }
    }
} 