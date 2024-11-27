use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Role {
    Tank,
    DPS,
    Support,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hero {
    pub name: String,
    pub role: Role,
    pub difficulty: u8, // 1-10
    pub abilities: Vec<String>,
    pub counters: Vec<String>,
    pub synergies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillMetric {
    pub aim_accuracy: f32,
    pub positioning: f32,
    pub ultimate_efficiency: f32,
    pub survival_rate: f32,
    pub team_coordination: f32,
}

#[derive(Debug)]
pub struct OverwatchTrainer {
    heroes: HashMap<String, Hero>,
    current_role: Role,
    skill_progress: SkillMetric,
    match_history: Vec<MatchResult>,
    training_schedule: Vec<TrainingTask>,
    rank_progress: RankProgress,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchResult {
    pub timestamp: DateTime<Utc>,
    pub hero: String,
    pub map: String,
    pub result: GameResult,
    pub performance_metrics: SkillMetric,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GameResult {
    Win,
    Loss,
    Draw,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingTask {
    pub focus_area: String,
    pub hero: String,
    pub completion_criteria: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RankProgress {
    pub current_rank: Rank,
    pub sr: u32,
    pub peak_sr: u32,
    pub games_played: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum Rank {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
    Master,
    Grandmaster,
    Top500,
}

impl OverwatchTrainer {
    pub fn new() -> Self {
        Self {
            heroes: Self::init_heroes(),
            current_role: Role::Support, // Start with support role (easier to learn)
            skill_progress: SkillMetric {
                aim_accuracy: 0.0,
                positioning: 0.0,
                ultimate_efficiency: 0.0,
                survival_rate: 0.0,
                team_coordination: 0.0,
            },
            match_history: Vec::new(),
            training_schedule: Vec::new(),
            rank_progress: RankProgress {
                current_rank: Rank::Bronze,
                sr: 1500,
                peak_sr: 1500,
                games_played: 0,
            },
        }
    }

    fn init_heroes() -> HashMap<String, Hero> {
        let mut heroes = HashMap::new();
        
        // Support Heroes
        heroes.insert("mercy".to_string(), Hero {
            name: "Mercy".to_string(),
            role: Role::Support,
            difficulty: 3,
            abilities: vec!["Healing Beam", "Damage Boost", "Guardian Angel", "Resurrect"].iter().map(String::from).collect(),
            counters: vec!["Widowmaker", "Genji", "Tracer"].iter().map(String::from).collect(),
            synergies: vec!["Pharah", "Ashe", "Echo"].iter().map(String::from).collect(),
        });

        heroes.insert("ana".to_string(), Hero {
            name: "Ana".to_string(),
            role: Role::Support,
            difficulty: 7,
            abilities: vec!["Sleep Dart", "Biotic Grenade", "Nano Boost"].iter().map(String::from).collect(),
            counters: vec!["Winston", "D.Va", "Genji"].iter().map(String::from).collect(),
            synergies: vec!["Reinhardt", "Genji", "Soldier: 76"].iter().map(String::from).collect(),
        });

        // Add all other supports...

        // Tank Heroes
        heroes.insert("reinhardt".to_string(), Hero {
            name: "Reinhardt".to_string(),
            role: Role::Tank,
            difficulty: 4,
            abilities: vec!["Barrier Field", "Charge", "Fire Strike", "Earthshatter"].iter().map(String::from).collect(),
            counters: vec!["Reaper", "Bastion", "Junkrat"].iter().map(String::from).collect(),
            synergies: vec!["Ana", "Lucio", "McCree"].iter().map(String::from).collect(),
        });

        // Add all tanks...

        // DPS Heroes
        heroes.insert("tracer".to_string(), Hero {
            name: "Tracer".to_string(),
            role: Role::DPS,
            difficulty: 8,
            abilities: vec!["Blink", "Recall", "Pulse Bomb"].iter().map(String::from).collect(),
            counters: vec!["McCree", "Brigitte", "Torbjorn"].iter().map(String::from).collect(),
            synergies: vec!["Winston", "D.Va", "Zenyatta"].iter().map(String::from).collect(),
        });

        // Add all DPS...

        heroes
    }

    // Add progression paths for each role
    fn get_role_progression(&self, role: &Role) -> Vec<&str> {
        match role {
            Role::Support => vec![
                "mercy",     // Easiest
                "moira",
                "lucio",
                "brigitte",
                "zenyatta",
                "baptiste",
                "ana",       // Hardest
                "kiriko",
            ],
            Role::Tank => vec![
                "reinhardt", // Easiest
                "orisa",
                "winston",
                "d.va",
                "sigma",
                "roadhog",
                "zarya",
                "wrecking_ball", // Hardest
            ],
            Role::DPS => vec![
                "soldier_76", // Easiest
                "reaper",
                "pharah",
                "junkrat",
                "mccree",
                "ashe",
                "hanzo",
                "widowmaker",
                "genji",
                "tracer",     // Hardest
            ],
        }
    }

    pub async fn train_hero(&mut self, hero: &str) -> Result<String, String> {
        if let Some(hero_data) = self.heroes.get(hero) {
            // Generate training tasks based on hero difficulty and role
            self.generate_training_tasks(hero_data);
            
            Ok(format!("Starting training for {}! Focus on: {}", 
                hero_data.name,
                self.training_schedule.first()
                    .map(|t| &t.focus_area)
                    .unwrap_or(&"basics".to_string())
            ))
        } else {
            Err("Hero not found".to_string())
        }
    }

    fn generate_training_tasks(&mut self, hero: &Hero) {
        self.training_schedule.clear();
        
        // Basic training tasks
        self.training_schedule.push(TrainingTask {
            focus_area: "Movement and Positioning".to_string(),
            hero: hero.name.clone(),
            completion_criteria: "Stay alive for entire match".to_string(),
            completed: false,
        });

        // Add role-specific tasks
        match hero.role {
            Role::Support => {
                self.training_schedule.push(TrainingTask {
                    focus_area: "Healing Priority".to_string(),
                    hero: hero.name.clone(),
                    completion_criteria: "10k healing per 10 minutes".to_string(),
                    completed: false,
                });
            },
            Role::DPS => {
                self.training_schedule.push(TrainingTask {
                    focus_area: "Aim Training".to_string(),
                    hero: hero.name.clone(),
                    completion_criteria: "50% accuracy in practice range".to_string(),
                    completed: false,
                });
            },
            Role::Tank => {
                self.training_schedule.push(TrainingTask {
                    focus_area: "Space Creation".to_string(),
                    hero: hero.name.clone(),
                    completion_criteria: "Block 15k damage per 10 minutes".to_string(),
                    completed: false,
                });
            },
        }
    }

    pub async fn update_progress(&mut self, match_result: MatchResult) {
        self.match_history.push(match_result.clone());
        self.update_skill_metrics(&match_result);
        self.update_rank(&match_result);
    }

    fn update_skill_metrics(&mut self, match_result: &MatchResult) {
        // Update skill progress based on match performance
        self.skill_progress = match_result.performance_metrics.clone();
        
        // Adjust training schedule based on performance
        self.adjust_training_schedule();
    }

    fn update_rank(&mut self, match_result: &MatchResult) {
        let sr_change = match match_result.result {
            GameResult::Win => 25,
            GameResult::Loss => -25,
            GameResult::Draw => 0,
        };

        self.rank_progress.sr = (self.rank_progress.sr as i32 + sr_change).max(0) as u32;
        self.rank_progress.games_played += 1;
        
        if self.rank_progress.sr > self.rank_progress.peak_sr {
            self.rank_progress.peak_sr = self.rank_progress.sr;
        }

        // Update rank based on SR
        self.rank_progress.current_rank = match self.rank_progress.sr {
            0..=1499 => Rank::Bronze,
            1500..=1999 => Rank::Silver,
            2000..=2499 => Rank::Gold,
            2500..=2999 => Rank::Platinum,
            3000..=3499 => Rank::Diamond,
            3500..=3999 => Rank::Master,
            4000..=4499 => Rank::Grandmaster,
            _ => Rank::Top500,
        };
    }

    fn adjust_training_schedule(&mut self) {
        // Remove completed tasks
        self.training_schedule.retain(|task| !task.completed);

        // Add new tasks based on current skill level
        if self.training_schedule.is_empty() {
            self.add_advanced_training_tasks();
        }
    }

    fn add_advanced_training_tasks(&mut self) {
        // Add more complex tasks as skills improve
        if self.skill_progress.aim_accuracy > 0.5 {
            self.training_schedule.push(TrainingTask {
                focus_area: "Advanced Aim Techniques".to_string(),
                hero: "Any".to_string(),
                completion_criteria: "70% accuracy with critical hits".to_string(),
                completed: false,
            });
        }
    }
} 