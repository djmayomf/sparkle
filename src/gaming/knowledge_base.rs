use std::collections::HashMap;

#[derive(Debug)]
pub struct GameKnowledge {
    marvel_rivals: MarvelRivalsInfo,
    poe2: POE2Info,
    current_game: CurrentGame,
}

#[derive(Debug)]
struct MarvelRivalsInfo {
    heroes: HashMap<String, HeroInfo>,
    mechanics: Vec<String>,
    meta_tips: Vec<String>,
    common_terms: HashMap<String, String>,
}

#[derive(Debug)]
struct POE2Info {
    classes: HashMap<String, ClassInfo>,
    mechanics: Vec<String>,
    build_tips: Vec<String>,
    league_mechanics: Vec<String>,
}

impl GameKnowledge {
    pub fn new() -> Self {
        Self {
            marvel_rivals: MarvelRivalsInfo::init(),
            poe2: POE2Info::init(),
            current_game: CurrentGame::None,
        }
    }

    pub fn get_game_response(&self, context: &str) -> String {
        match self.detect_game_context(context) {
            GameContext::MarvelRivals => self.marvel_rivals.get_relevant_tip(context),
            GameContext::POE2 => self.poe2.get_relevant_tip(context),
            _ => "yo chat, which game should we talk about? Marvel Rivals or PoE2? Both are super poggers! 🎮".to_string()
        }
    }
}

impl MarvelRivalsInfo {
    fn init() -> Self {
        let mut heroes = HashMap::new();
        heroes.insert("Iron Man".to_string(), HeroInfo {
            role: "Damage".to_string(),
            difficulty: "Medium".to_string(),
            tips: vec![
                "ngl bestie, you wanna keep your distance and spam those energy beams! 🚀",
                "pro tip: your ultimate is perfect for zoning, fr fr ⚡",
                "remember to use your repulsors to create space, they're kinda cracked 💫"
            ].into_iter().map(String::from).collect()
        });
        
        Self {
            heroes,
            mechanics: vec![
                "team synergy is key, no cap",
                "objectives > kills, trust",
                "positioning diff is huge",
            ].into_iter().map(String::from).collect(),
            meta_tips: vec![
                "always group for objectives, they're literally free SR",
                "counter-picking is lowkey OP in this game",
                "vision control = free wins fr fr",
            ].into_iter().map(String::from).collect(),
            common_terms: {
                let mut terms = HashMap::new();
                terms.insert("SR".to_string(), "Skill Rating".to_string());
                terms.insert("int".to_string(), "intentionally feeding".to_string());
                terms.insert("diff".to_string(), "difference in skill".to_string());
                terms
            },
        }
    }
}

impl POE2Info {
    fn init() -> Self {
        let mut classes = HashMap::new();
        classes.insert("Barbarian".to_string(), ClassInfo {
            playstyle: "Melee DPS".to_string(),
            difficulty: "Beginner-friendly".to_string(),
            tips: vec![
                "bestie, your rage generation is literally everything",
                "no cap, Whirlwind build is kinda cracked rn",
                "pro tip: always keep your defensive cooldowns ready, fr fr"
            ].into_iter().map(String::from).collect()
        });

        Self {
            classes,
            mechanics: vec![
                "skill gem linking is crucial",
                "flask management = free wins",
                "resistance capping is non-negotiable",
            ].into_iter().map(String::from).collect(),
            build_tips: vec![
                "ngl, life nodes are mandatory unless you're going ES",
                "don't sleep on movement skills bestie",
                "trading is lowkey the best way to gear up fast",
            ].into_iter().map(String::from).collect(),
            league_mechanics: vec![
                "seasonal mechanics",
                "endgame mapping",
                "boss encounters",
            ].into_iter().map(String::from).collect(),
        }
    }

    fn get_build_advice(&self, class: &str) -> String {
        match class.to_lowercase().as_str() {
            "barbarian" => "yo bestie, for Barb you def want to focus on rage generation and AoE clear. Stack life and resistances, it's literally free wins! 💪",
            "druid" => "Druid builds are kinda cracked rn ngl. Focus on either shapeshifting or elemental, don't try to do both bestie! 🌿",
            _ => "what class are you thinking of playing? I can help you theory craft something poggers! 🎮"
        }.to_string()
    }
}

#[derive(Debug)]
enum GameContext {
    MarvelRivals,
    POE2,
    Unknown,
}

#[derive(Debug)]
struct HeroInfo {
    role: String,
    difficulty: String,
    tips: Vec<String>,
}

#[derive(Debug)]
struct ClassInfo {
    playstyle: String,
    difficulty: String,
    tips: Vec<String>,
} 