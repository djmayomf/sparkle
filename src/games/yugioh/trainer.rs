use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeckArchetype {
    pub name: String,
    pub tier: DeckTier,
    pub play_style: PlayStyle,
    pub core_cards: Vec<String>,
    pub tech_choices: Vec<String>,
    pub counters: Vec<String>,
    pub combo_lines: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeckTier {
    Meta,
    RogueCompetitive,
    Rogue,
    Casual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayStyle {
    Combo,
    Control,
    Midrange,
    Aggro,
    StunLockdown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DuelStats {
    pub wins: u32,
    pub losses: u32,
    pub rank: String,
    pub favorite_deck: String,
    pub gems_earned: u32,
    pub crafting_points: HashMap<CardRarity, u32>,
}

#[derive(Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CardRarity {
    Normal,
    Rare,
    SuperRare,
    UltraRare,
}

pub struct YugiohTrainer {
    decks: HashMap<String, DeckArchetype>,
    current_deck: Option<String>,
    duel_stats: DuelStats,
    known_combos: Vec<String>,
    meta_knowledge: HashMap<String, Vec<String>>, // matchup -> counter plays
}

impl YugiohTrainer {
    pub fn new() -> Self {
        Self {
            decks: Self::init_meta_decks(),
            current_deck: None,
            duel_stats: DuelStats {
                wins: 0,
                losses: 0,
                rank: "Rookie".to_string(),
                favorite_deck: String::new(),
                gems_earned: 0,
                crafting_points: HashMap::new(),
            },
            known_combos: Vec::new(),
            meta_knowledge: HashMap::new(),
        }
    }

    fn init_meta_decks() -> HashMap<String, DeckArchetype> {
        let mut decks = HashMap::new();

        // Meta Decks
        decks.insert("dragonlink".to_string(), DeckArchetype {
            name: "Dragon Link".to_string(),
            tier: DeckTier::Meta,
            play_style: PlayStyle::Combo,
            core_cards: vec![
                "Quick Launch".to_string(),
                "Striker Dragon".to_string(),
                "Chaos Space".to_string(),
            ],
            tech_choices: vec![
                "Called by the Grave".to_string(),
                "Crossout Designator".to_string(),
            ],
            counters: vec![
                "Nibiru".to_string(),
                "Ash Blossom".to_string(),
            ],
            combo_lines: vec![
                "Normal Summon Rokket Tracer -> Link into Striker Dragon...".to_string(),
            ],
        });

        // Add more meta decks...
        decks
    }

    pub async fn purchase_packs(&mut self, pack_type: &str, amount: u32) -> Result<Vec<String>, String> {
        if self.duel_stats.gems_earned < amount * 100 {
            return Err("Insufficient gems! Complete more duels to earn gems.".to_string());
        }

        // Simulate pack opening
        let mut cards = Vec::new();
        for _ in 0..amount {
            self.duel_stats.gems_earned -= 100;
            cards.extend(self.simulate_pack_opening());
        }

        Ok(cards)
    }

    fn simulate_pack_opening(&mut self) -> Vec<String> {
        let mut cards = Vec::new();
        // Guaranteed rarity distribution
        cards.push(self.generate_card(CardRarity::UltraRare));
        cards.push(self.generate_card(CardRarity::SuperRare));
        cards.extend((0..6).map(|_| self.generate_card(CardRarity::Rare)));

        cards
    }

    fn generate_card(&self, rarity: CardRarity) -> String {
        // Simplified card generation based on rarity
        match rarity {
            CardRarity::UltraRare => "Premium Meta Card".to_string(),
            CardRarity::SuperRare => "Strong Tech Card".to_string(),
            CardRarity::Rare => "Common Staple".to_string(),
            CardRarity::Normal => "Basic Card".to_string(),
        }
    }

    pub async fn learn_deck(&mut self, deck_name: &str) -> Result<String, String> {
        if let Some(deck) = self.decks.get(deck_name) {
            self.current_deck = Some(deck_name.to_string());
            self.known_combos = deck.combo_lines.clone();

            Ok(format!("Learning {} deck. Playstyle: {:?}. Starting with basic combos...", 
                deck.name, deck.play_style))
        } else {
            Err("Deck not found in current meta".to_string())
        }
    }

    pub async fn practice_combo(&mut self, combo_name: &str) -> Result<(), String> {
        if let Some(deck_name) = &self.current_deck {
            if let Some(deck) = self.decks.get(deck_name) {
                if deck.combo_lines.iter().any(|combo| combo.contains(combo_name)) {
                    // Simulate combo practice
                    println!("Practicing combo: {}", combo_name);
                    // Add success rate tracking
                    return Ok(());
                }
            }
        }
        Err("Combo not found or no deck selected".to_string())
    }

    pub async fn record_duel_result(&mut self, won: bool, opponent_deck: &str) -> Result<(), String> {
        if won {
            self.duel_stats.wins += 1;
            self.duel_stats.gems_earned += 50; // Bonus gems for winning
        } else {
            self.duel_stats.losses += 1;
            self.duel_stats.gems_earned += 10; // Consolation gems
        }

        // Update rank based on win rate
        self.update_rank();

        // Learn from the duel
        if let Some(current_deck) = &self.current_deck {
            let matchup_key = format!("{}_{}", current_deck, opponent_deck);
            let counters = self.meta_knowledge
                .entry(matchup_key)
                .or_insert_with(Vec::new);

            if !won {
                // Learn new counter strategy
                counters.push("New counter strategy learned from loss".to_string());
            }
        }

        Ok(())
    }

    fn update_rank(&mut self) {
        let total_duels = self.duel_stats.wins + self.duel_stats.losses;
        if total_duels == 0 {
            return;
        }

        let win_rate = self.duel_stats.wins as f32 / total_duels as f32;
        self.duel_stats.rank = match (win_rate, total_duels) {
            (w, t) if w >= 0.65 && t >= 100 => "Diamond".to_string(),
            (w, t) if w >= 0.60 && t >= 50 => "Platinum".to_string(),
            (w, t) if w >= 0.55 && t >= 30 => "Gold".to_string(),
            (w, t) if w >= 0.50 && t >= 10 => "Silver".to_string(),
            _ => "Bronze".to_string(),
        };
    }

    pub fn get_deck_recommendation(&self, play_style: PlayStyle) -> Vec<&DeckArchetype> {
        self.decks.values()
            .filter(|deck| deck.play_style == play_style)
            .collect()
    }

    pub fn get_crafting_costs(&self, card_name: &str) -> HashMap<CardRarity, u32> {
        // Simplified crafting system
        let mut costs = HashMap::new();
        costs.insert(CardRarity::UltraRare, 30);
        costs.insert(CardRarity::SuperRare, 20);
        costs.insert(CardRarity::Rare, 10);
        costs
    }
} 