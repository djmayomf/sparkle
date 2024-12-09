use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Build {
    pub class: String,
    pub main_skills: Vec<String>,
    pub passive_tree: Vec<String>,
    pub gear_requirements: Vec<GearPiece>,
    pub leveling_path: Vec<String>,
    pub difficulty: u8, // 1-10
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GearPiece {
    pub slot: String,
    pub requirements: Vec<String>,
    pub recommended_mods: Vec<String>,
    pub crafting_steps: Vec<String>,
}

pub struct POE2Trainer {
    builds: HashMap<String, Build>,
    current_build: Option<String>,
    progression: CharacterProgression,
    crafting_knowledge: CraftingSystem,
    league_mechanics: LeagueMechanics,
}

impl POE2Trainer {
    pub fn new() -> Self {
        Self {
            builds: Self::init_builds(),
            current_build: None,
            progression: CharacterProgression::new(),
            crafting_knowledge: CraftingSystem::new(),
            league_mechanics: LeagueMechanics::new(),
        }
    }

    fn init_builds() -> HashMap<String, Build> {
        let mut builds = HashMap::new();
        
        builds.insert("whirlwind_barb".to_string(), Build {
            class: "Barbarian".to_string(),
            main_skills: vec![
                "Whirlwind",
                "Battle Cry",
                "War Banner"
            ].iter().map(String::from).collect(),
            passive_tree: vec![
                "Life nodes",
                "Attack Speed",
                "Physical Damage"
            ].iter().map(String::from).collect(),
            gear_requirements: vec![
                GearPiece {
                    slot: "Weapon".to_string(),
                    requirements: vec!["High pDPS"].iter().map(String::from).collect(),
                    recommended_mods: vec!["Attack Speed", "Physical Damage"].iter().map(String::from).collect(),
                    crafting_steps: vec!["Alt spam for T1 phys", "Regal", "Multimod"].iter().map(String::from).collect(),
                }
            ],
            leveling_path: vec![
                "Get Whirlwind ASAP",
                "Focus on life nodes early",
                "Grab key damage nodes"
            ].iter().map(String::from).collect(),
            difficulty: 6,
        });

        builds
    }

    pub async fn learn_build(&mut self, build_name: &str) -> Result<String, String> {
        if let Some(build) = self.builds.get(build_name) {
            self.current_build = Some(build_name.to_string());
            Ok(format!("yo bestie, let's learn this {} build! It's kinda cracked fr fr 🎮\n\
                      Main skills: {}\n\
                      Difficulty: {}/10", 
                build.class,
                build.main_skills.join(", "),
                build.difficulty
            ))
        } else {
            Err("that build isn't in the meta rn bestie! 😅".to_string())
        }
    }

    pub async fn get_crafting_advice(&self, item_type: &str) -> String {
        match item_type {
            "weapon" => "bestie, you def want to alt spam for T1 phys, then regal and pray fr fr 🙏".to_string(),
            "armor" => "life and resists are literally free wins, just essence spam tbh 💫".to_string(),
            _ => "what are you trying to craft? lmk and I'll help you make it poggers! 🛠️".to_string()
        }
    }
} 