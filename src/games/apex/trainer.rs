use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Legend {
    pub name: String,
    pub class: LegendClass,
    pub abilities: Vec<Ability>,
    pub difficulty: u8, // 1-10
    pub team_synergies: Vec<String>,
    pub preferred_loadouts: Vec<Loadout>,
    pub movement_tech: Vec<MovementTechnique>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LegendClass {
    Assault,
    Skirmisher,
    Recon,
    Controller,
    Support,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ability {
    pub name: String,
    pub key: String,
    pub cooldown: u32,
    pub usage_tips: Vec<String>,
    pub combo_potential: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Loadout {
    pub primary: Weapon,
    pub secondary: Weapon,
    pub preferred_attachments: Vec<String>,
    pub playstyle: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weapon {
    pub name: String,
    pub ammo_type: AmmoType,
    pub recoil_pattern: Vec<Position>,
    pub optimal_range: Range,
    pub dps_stats: DPSStats,
}

#[derive(Debug)]
pub struct ApexTrainer {
    legends: HashMap<String, Legend>,
    current_legend: Option<String>,
    movement_trainer: MovementTrainer,
    combat_trainer: CombatTrainer,
    strategy_engine: StrategyEngine,
    match_history: Vec<MatchResult>,
    rank_progress: RankProgress,
}

impl ApexTrainer {
    pub fn new() -> Self {
        Self {
            legends: Self::init_legends(),
            current_legend: None,
            movement_trainer: MovementTrainer::new(),
            combat_trainer: CombatTrainer::new(),
            strategy_engine: StrategyEngine::new(),
            match_history: Vec::new(),
            rank_progress: RankProgress::default(),
        }
    }

    fn init_legends() -> HashMap<String, Legend> {
        let mut legends = HashMap::new();
        
        // Add Wraith
        legends.insert("wraith".to_string(), Legend {
            name: "Wraith".to_string(),
            class: LegendClass::Assault,
            abilities: vec![
                Ability {
                    name: "Into the Void".to_string(),
                    key: "Q".to_string(),
                    cooldown: 25,
                    usage_tips: vec![
                        "Use for repositioning".to_string(),
                        "Can escape thermites and arc stars".to_string(),
                    ],
                    combo_potential: vec!["Portal for team rotation".to_string()],
                },
                // Add other abilities...
            ],
            difficulty: 8,
            team_synergies: vec!["Gibraltar", "Bloodhound"].iter().map(String::from).collect(),
            preferred_loadouts: vec![
                Loadout {
                    primary: Weapon {
                        name: "R-301".to_string(),
                        ammo_type: AmmoType::Light,
                        recoil_pattern: vec![/* recoil points */],
                        optimal_range: Range { min: 10.0, max: 50.0 },
                        dps_stats: DPSStats::default(),
                    },
                    secondary: Weapon {
                        name: "Peacekeeper".to_string(),
                        ammo_type: AmmoType::Shotgun,
                        recoil_pattern: vec![],
                        optimal_range: Range { min: 0.0, max: 15.0 },
                        dps_stats: DPSStats::default(),
                    },
                    preferred_attachments: vec!["3x HCOG", "Purple Stock"].iter().map(String::from).collect(),
                    playstyle: "Aggressive entry fragger".to_string(),
                }
            ],
            movement_tech: vec![
                MovementTechnique {
                    name: "Super Glide".to_string(),
                    difficulty: 8,
                    instructions: vec!["Jump at end of climb".to_string(), "Crouch immediately after".to_string()],
                },
                // Add other movement techniques...
            ],
        });

        legends
    }

    pub async fn train_movement(&mut self, technique: &str) -> Result<MovementStats, String> {
        if let Some(legend) = self.current_legend.as_ref()
            .and_then(|l| self.legends.get(l)) 
        {
            self.movement_trainer.practice_technique(
                technique,
                &legend.movement_tech
            ).await
        } else {
            Err("No legend selected".to_string())
        }
    }

    pub async fn practice_recoil(&mut self, weapon: &str) -> Result<RecoilStats, String> {
        self.combat_trainer.practice_recoil_control(weapon).await
    }

    pub async fn analyze_match(&mut self, match_data: MatchData) -> Result<MatchAnalysis, String> {
        // Record match
        self.match_history.push(match_data.clone());

        // Analyze performance
        let movement_analysis = self.movement_trainer.analyze_movement(&match_data);
        let combat_analysis = self.combat_trainer.analyze_combat(&match_data);
        let strategy_analysis = self.strategy_engine.analyze_decisions(&match_data);

        // Update rank progress
        self.rank_progress.update(&match_data);

        Ok(MatchAnalysis {
            movement_stats: movement_analysis,
            combat_stats: combat_analysis,
            strategic_decisions: strategy_analysis,
            improvement_areas: self.identify_improvement_areas(
                &movement_analysis,
                &combat_analysis,
                &strategy_analysis
            ),
        })
    }

    pub async fn get_training_routine(&self) -> TrainingRoutine {
        TrainingRoutine {
            movement_practice: vec![
                "Practice tap strafing for 10 minutes".to_string(),
                "Super glide training".to_string(),
                "Wall bounce practice".to_string(),
            ],
            aim_training: vec![
                "Recoil control: R-301".to_string(),
                "Flick shots: Wingman".to_string(),
                "Tracking: CAR SMG".to_string(),
            ],
            game_sense: vec![
                "Review 2 pro VODs".to_string(),
                "Practice rotations on current map".to_string(),
                "Work on positioning".to_string(),
            ],
        }
    }
}

#[derive(Debug)]
struct MovementTrainer {
    technique_progress: HashMap<String, f32>,
    movement_scores: Vec<MovementScore>,
}

#[derive(Debug)]
struct CombatTrainer {
    weapon_accuracies: HashMap<String, Vec<f32>>,
    damage_stats: Vec<DamageStats>,
    recoil_patterns: HashMap<String, Vec<Position>>,
}

#[derive(Debug)]
struct StrategyEngine {
    rotation_knowledge: HashMap<String, Vec<RotationPath>>,
    position_analysis: PositionAnalyzer,
    engagement_rules: Vec<EngagementRule>,
} 