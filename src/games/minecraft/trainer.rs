use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CraftingRecipe {
    pub name: String,
    pub ingredients: Vec<String>,
    pub pattern: Option<Vec<String>>,
    pub result: String,
    pub quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub level: u32,
    pub experience: f32,
    pub milestones: Vec<String>,
}

#[derive(Debug)]
pub struct MinecraftTrainer {
    crafting_knowledge: HashMap<String, CraftingRecipe>,
    skills: HashMap<String, Skill>,
    current_objectives: Vec<GameObjective>,
    completed_achievements: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameObjective {
    pub name: String,
    pub description: String,
    pub requirements: Vec<String>,
    pub completed: bool,
    pub priority: u32,
}

impl MinecraftTrainer {
    pub fn new() -> Self {
        Self {
            crafting_knowledge: Self::init_crafting(),
            skills: Self::init_skills(),
            current_objectives: Vec::new(),
            completed_achievements: Vec::new(),
        }
    }

    fn init_crafting() -> HashMap<String, CraftingRecipe> {
        let mut recipes = HashMap::new();
        
        recipes.insert("crafting_table".to_string(), CraftingRecipe {
            name: "Crafting Table".to_string(),
            ingredients: vec!["Wooden Planks".to_string()],
            pattern: Some(vec![
                "XX".to_string(),
                "XX".to_string(),
            ]),
            result: "Crafting Table".to_string(),
            quantity: 1,
        });

        // Add more recipes...
        recipes
    }

    fn init_skills() -> HashMap<String, Skill> {
        let mut skills = HashMap::new();
        
        skills.insert("building".to_string(), Skill {
            name: "Building".to_string(),
            level: 1,
            experience: 0.0,
            milestones: vec![
                "Build first house".to_string(),
                "Create automated farm".to_string(),
                "Build village trading hall".to_string(),
            ],
        });

        // Add more skills...
        skills
    }

    pub async fn learn_recipe(&mut self, recipe_name: &str) -> Result<String, String> {
        if let Some(recipe) = self.crafting_knowledge.get(recipe_name) {
            Ok(format!("Learned how to craft {}! You need: {}", 
                recipe.name,
                recipe.ingredients.join(", ")
            ))
        } else {
            Err("Recipe not found".to_string())
        }
    }

    pub async fn set_objectives(&mut self, game_mode: &str) {
        self.current_objectives.clear();
        
        match game_mode {
            "survival" => {
                self.add_survival_objectives();
            }
            "creative" => {
                self.add_creative_objectives();
            }
            "hardcore" => {
                self.add_hardcore_objectives();
            }
            _ => {
                self.add_basic_objectives();
            }
        }
    }

    fn add_survival_objectives(&mut self) {
        self.current_objectives.extend(vec![
            GameObjective {
                name: "Basic Shelter".to_string(),
                description: "Build a safe shelter before night".to_string(),
                requirements: vec!["Wood".to_string(), "Crafting Table".to_string()],
                completed: false,
                priority: 1,
            },
            GameObjective {
                name: "Food Source".to_string(),
                description: "Create a sustainable food source".to_string(),
                requirements: vec!["Seeds".to_string(), "Hoe".to_string(), "Water".to_string()],
                completed: false,
                priority: 2,
            },
        ]);
    }

    pub async fn update_skill(&mut self, skill_name: &str, experience: f32) -> Result<String, String> {
        if let Some(skill) = self.skills.get_mut(skill_name) {
            skill.experience += experience;
            
            // Level up if enough experience
            if skill.experience >= (skill.level as f32 * 100.0) {
                skill.level += 1;
                return Ok(format!("Level up! {} is now level {}", skill.name, skill.level));
            }
            
            Ok(format!("Gained {} experience in {}", experience, skill.name))
        } else {
            Err("Skill not found".to_string())
        }
    }

    pub fn get_next_objective(&self) -> Option<&GameObjective> {
        self.current_objectives.iter()
            .filter(|obj| !obj.completed)
            .min_by_key(|obj| obj.priority)
    }

    pub async fn complete_objective(&mut self, objective_name: &str) -> Result<String, String> {
        if let Some(objective) = self.current_objectives.iter_mut()
            .find(|obj| obj.name == objective_name) {
            objective.completed = true;
            Ok(format!("Completed objective: {}!", objective.name))
        } else {
            Err("Objective not found".to_string())
        }
    }
} 