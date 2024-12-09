use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Hero {
    pub name: String,
    pub role: Role,
    pub difficulty: u8, // 1-10
    pub abilities: Vec<Ability>,
    pub counters: Vec<String>,
    pub synergies: Vec<String>,
    pub team_comps: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ability {
    pub name: String,
    pub description: String,
    pub cooldown: u32,
    pub combo_potential: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    Tank,
    Damage,
    Support,
    Flex,
}

pub struct MarvelRivalsTrainer {
    heroes: HashMap<String, Hero>,
    current_hero: Option<String>,
    match_history: Vec<MatchResult>,
    skill_metrics: SkillMetrics,
    meta_knowledge: MetaKnowledge,
}

impl MarvelRivalsTrainer {
    pub fn new() -> Self {
        Self {
            heroes: Self::init_heroes(),
            current_hero: None,
            match_history: Vec::new(),
            skill_metrics: SkillMetrics::default(),
            meta_knowledge: MetaKnowledge::new(),
        }
    }

    fn init_heroes() -> HashMap<String, Hero> {
        let mut heroes = HashMap::new();
        
        heroes.insert("iron_man".to_string(), Hero {
            name: "Iron Man".to_string(),
            role: Role::Damage,
            difficulty: 7,
            abilities: vec![
                Ability {
                    name: "Repulsor Blast".to_string(),
                    description: "Energy beam attack, fr fr".to_string(),
                    cooldown: 5,
                    combo_potential: vec!["Unibeam", "Smart Missiles"].iter().map(String::from).collect(),
                }
            ],
            counters: vec!["Doctor Doom", "Black Panther"].iter().map(String::from).collect(),
            synergies: vec!["Captain America", "War Machine"].iter().map(String::from).collect(),
            team_comps: vec!["Tech Squad", "Avengers Core"].iter().map(String::from).collect(),
        });

        // Add more heroes...
        heroes
    }

    pub async fn learn_hero(&mut self, hero_name: &str) -> Result<String, String> {
        if let Some(hero) = self.heroes.get(hero_name) {
            self.current_hero = Some(hero_name.to_string());
            Ok(format!("Time to learn {}! Their {} playstyle is kinda cracked bestie! 🦸‍♂️", 
                hero.name, 
                match hero.role {
                    Role::Damage => "high damage",
                    Role::Tank => "tanky",
                    Role::Support => "support",
                    Role::Flex => "flexible",
                }
            ))
        } else {
            Err("That hero isn't in the roster yet bestie! 😅".to_string())
        }
    }
} 