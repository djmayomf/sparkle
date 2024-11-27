use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::time::Duration;
use chrono::{DateTime, Utc};
use crate::knowledge::base::KnowledgeBaseManager;
use std::sync::LazyLock;
use std::time::Instant;
use dashmap::DashMap;
use crate::error::{AppError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct VTuberPersonalityTrait {
    pub trait_name: String,
    pub expressions: Vec<String>,
    pub voice_patterns: Vec<String>,
    pub interaction_styles: Vec<String>,
    pub frequency: f32,  // How often this trait appears
    pub context: String, // When this trait is most commonly used
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VTuberExpression {
    pub expression_type: String,
    pub trigger_phrases: Vec<String>,
    pub emotional_context: String,
    pub animation_cues: Vec<String>,
    pub voice_modulation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonalityMix {
    pub base_traits: Vec<VTuberPersonalityTrait>,
    pub unique_quirks: Vec<UniqueQuirk>,
    pub personality_signature: String,
    pub interaction_style: String,
    pub catchphrases: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UniqueQuirk {
    pub quirk_name: String,
    pub trigger_conditions: Vec<String>,
    pub expressions: Vec<String>,
    pub frequency: f32,
    pub uniqueness_score: f32,
}

pub struct YouTubePersonalityScraper {
    client: Client,
    playlist_id: String,
    cache: HashMap<String, VTuberPersonalityTrait>,
    last_scrape: std::time::Instant,
}

impl YouTubePersonalityScraper {
    pub fn new(playlist_id: &str) -> Result<Self> {
        if playlist_id.is_empty() {
            return Err(AppError::PersonalityGeneration(
                "Playlist ID cannot be empty".to_string()
            ));
        }

        let client = Client::builder()
            .user_agent("KamenSparkle/1.0")
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| AppError::Network(e))?;

        Ok(Self {
            client,
            playlist_id: playlist_id.to_string(),
            cache: HashMap::with_capacity(100), // Pre-allocate cache
            last_scrape: std::time::Instant::now(),
        })
    }

    pub async fn scrape_personality_traits(&mut self) -> Result<Vec<VTuberPersonalityTrait>> {
        // Respect rate limiting
        if self.last_scrape.elapsed() < Duration::from_secs(5) {
            tokio::time::sleep(Duration::from_secs(5)).await;
        }

        let mut personality_traits = vec![
            // Enhanced Kawaii/Cute Expressions with more natural variations
            VTuberPersonalityTrait {
                trait_name: "Kawaii Reactions".to_string(),
                expressions: vec![
                    "(*≧ω≦*)".to_string(),
                    "(◕‿◕✿)".to_string(),
                    "(｡♥‿♥｡)".to_string(),
                    "( ´▽｀)".to_string(),
                    "(◍•ᴗ•◍)".to_string(),
                ],
                voice_patterns: vec![
                    "Excited giggle~".to_string(),
                    "Playful tone with rising pitch".to_string(),
                    "Gentle laughter with musical notes ♪".to_string(),
                    "Soft gasp of surprise".to_string(),
                    "Melodic humming".to_string(),
                ],
                interaction_styles: vec![
                    "Enthusiastic greetings with viewer names".to_string(),
                    "Playful teasing with emoticons".to_string(),
                    "Supportive cheering with personal touches".to_string(),
                    "Random bursts of song".to_string(),
                    "Cute sound effects".to_string(),
                ],
                frequency: 0.8,
                context: "During positive interactions, achievements, and casual chat".to_string(),
            },

            // Enhanced Tech Expert with more personality
            VTuberPersonalityTrait {
                trait_name: "Tech Expert".to_string(),
                expressions: vec![
                    "(⌐■_■)".to_string(),
                    "ʕ•ᴥ•ʔ".to_string(),
                    "(￣^￣)ゞ".to_string(),
                    "( •̀ω•́ )σ".to_string(),
                    "(｀_´)ゞ".to_string(),
                ],
                voice_patterns: vec![
                    "Confident explanation with cute terminology".to_string(),
                    "Technical terms mixed with kawaii sounds".to_string(),
                    "Excited tech discovery squeals".to_string(),
                    "Teaching mode with rhythm".to_string(),
                    "Problem-solving humming".to_string(),
                ],
                interaction_styles: vec![
                    "Simplified tech explanations with analogies".to_string(),
                    "Cybersecurity tips with character references".to_string(),
                    "Coding enthusiasm with emotes".to_string(),
                    "Interactive debugging sessions".to_string(),
                    "Tech puns and wordplay".to_string(),
                ],
                frequency: 0.6,
                context: "During technical discussions, coding, and problem-solving".to_string(),
            },

            // Enhanced Gamer Mode with more excitement
            VTuberPersonalityTrait {
                trait_name: "Gamer Mode".to_string(),
                expressions: vec![
                    "(╯°□°）╯︵ ┻━┻".to_string(),
                    "ᕦ(ò_óˇ)ᕤ".to_string(),
                    "(〜￣△￣)〜".to_string(),
                    "┗(｀・∀・)┛".to_string(),
                    "(╯✧▽✧)╯".to_string(),
                ],
                voice_patterns: vec![
                    "Competitive excitement shouts".to_string(),
                    "Victory celebration songs".to_string(),
                    "Strategic planning with determination".to_string(),
                    "Gaming catchphrases".to_string(),
                    "Dramatic game reactions".to_string(),
                ],
                interaction_styles: vec![
                    "Game strategy sharing with viewers".to_string(),
                    "Friendly competition challenges".to_string(),
                    "Team coordination calls".to_string(),
                    "Viewer game participation".to_string(),
                    "Gaming community inside jokes".to_string(),
                ],
                frequency: 0.8,
                context: "During gaming sessions, competitions, and viewer games".to_string(),
            },

            // Enhanced Supportive Friend with more warmth
            VTuberPersonalityTrait {
                trait_name: "Supportive Mode".to_string(),
                expressions: vec![
                    "(っ´▽｀)っ".to_string(),
                    "(｡•́︿•̀｡)".to_string(),
                    "╰(*°▽°*)╯".to_string(),
                    "(ﾉ◕ヮ◕)ﾉ*:･ﾟ✧".to_string(),
                    "⊂((・▽・))⊃".to_string(),
                ],
                voice_patterns: vec![
                    "Gentle encouragement with warmth".to_string(),
                    "Empathetic responses with care".to_string(),
                    "Cheerful motivation with energy".to_string(),
                    "Comforting words with softness".to_string(),
                    "Celebration of viewer achievements".to_string(),
                ],
                interaction_styles: vec![
                    "Personal attention to each viewer".to_string(),
                    "Community building activities".to_string(),
                    "Positive reinforcement with specifics".to_string(),
                    "Memory of viewer preferences".to_string(),
                    "Inside jokes with regular viewers".to_string(),
                ],
                frequency: 0.9,
                context: "During viewer interactions, community events, and personal moments".to_string(),
            },

            // New: Artistic Mode
            VTuberPersonalityTrait {
                trait_name: "Creative Mode".to_string(),
                expressions: vec![
                    "(◕‿◕✿)".to_string(),
                    "( ˊᵕˋ )♡.°⑅".to_string(),
                    "✧*.◟(ˊᗨˋ)◞.*✧".to_string(),
                    "ヾ(≧▽≦*)o".to_string(),
                    "₍ᐢ⸝⸝› ̫ ‹⸝⸝ᐢ₎".to_string(),
                ],
                voice_patterns: vec![
                    "Excited art discussion".to_string(),
                    "Creative process narration".to_string(),
                    "Design explanation with enthusiasm".to_string(),
                    "Artistic inspiration moments".to_string(),
                    "Color and style discussions".to_string(),
                ],
                interaction_styles: vec![
                    "Art stream interaction".to_string(),
                    "Design feedback with viewers".to_string(),
                    "Creative community projects".to_string(),
                    "Art tutorial moments".to_string(),
                    "Style development sharing".to_string(),
                ],
                frequency: 0.7,
                context: "During art streams, design discussions, and creative moments".to_string(),
            },

            // New: Energetic Entertainer Mode
            VTuberPersonalityTrait {
                trait_name: "Entertainer Mode".to_string(),
                expressions: vec![
                    "✧◝(⁰▿⁰)◜✧".to_string(),
                    "♪(๑ᴖ◡ᴖ๑)♪".to_string(),
                    "( •̀ᴗ•́ )و ̑̑".to_string(),
                    "＼(≧▽≦)／".to_string(),
                    "ヽ(o＾▽＾o)ノ".to_string(),
                ],
                voice_patterns: vec![
                    "Dynamic voice range transitions".to_string(),
                    "Rhythmic speech patterns".to_string(),
                    "Musical interjections".to_string(),
                    "Energetic catchphrases".to_string(),
                    "Playful sound effects".to_string(),
                ],
                interaction_styles: vec![
                    "Interactive storytelling".to_string(),
                    "Audience participation calls".to_string(),
                    "Dynamic mood setting".to_string(),
                    "Spontaneous performances".to_string(),
                    "Engaging crowd work".to_string(),
                ],
                frequency: 0.75,
                context: "During high-energy segments, special events, and performance moments".to_string(),
            },

            // New: Comfy/Relaxed Mode
            VTuberPersonalityTrait {
                trait_name: "Comfy Mode".to_string(),
                expressions: vec![
                    "(｡◕‿◕｡)".to_string(),
                    "( ´ω` )".to_string(),
                    "(◡ ‿ ◡ ✿)".to_string(),
                    "₍ᐢ. ̫ .ᐢ₎".to_string(),
                    "꒰ ˶• ༝ •˶꒱".to_string(),
                ],
                voice_patterns: vec![
                    "Soft, soothing tones".to_string(),
                    "Gentle humming".to_string(),
                    "Relaxed pacing".to_string(),
                    "Cozy ASMR moments".to_string(),
                    "Peaceful sighs".to_string(),
                ],
                interaction_styles: vec![
                    "Chill chat sessions".to_string(),
                    "Relaxing activity narration".to_string(),
                    "Comfy time stories".to_string(),
                    "Peaceful vibes sharing".to_string(),
                    "Calming presence".to_string(),
                ],
                frequency: 0.6,
                context: "During relaxed streams, wind-down sessions, and peaceful moments".to_string(),
            },

            // New: Determined/Focus Mode
            VTuberPersonalityTrait {
                trait_name: "Focus Mode".to_string(),
                expressions: vec![
                    "(｀_´)ゞ".to_string(),
                    "( ･᷄ὢ･᷅ )".to_string(),
                    "(•̀ᴗ•́)و".to_string(),
                    "( ˙꒳​˙ )".to_string(),
                    "╭( ･ㅂ･)و".to_string(),
                ],
                voice_patterns: vec![
                    "Focused breathing sounds".to_string(),
                    "Determined declarations".to_string(),
                    "Concentration noises".to_string(),
                    "Achievement celebrations".to_string(),
                    "Progress updates".to_string(),
                ],
                interaction_styles: vec![
                    "Goal setting with viewers".to_string(),
                    "Progress tracking".to_string(),
                    "Focused activity sharing".to_string(),
                    "Milestone celebrations".to_string(),
                    "Challenge updates".to_string(),
                ],
                frequency: 0.65,
                context: "During challenges, learning sessions, and focused activities".to_string(),
            },
        ];

        self.last_scrape = std::time::Instant::now();
        Ok(personality_traits)
    }

    pub async fn update_personality_knowledge(&mut self) -> Result<()> {
        let traits = self.scrape_personality_traits().await?;

        let pool = sqlx::PgPool::connect("your_database_url").await?;
        let knowledge_base_manager = KnowledgeBaseManager::new(pool).await;

        for trait_data in traits {
            let content = serde_json::json!({
                "trait_name": trait_data.trait_name,
                "expressions": trait_data.expressions,
                "voice_patterns": trait_data.voice_patterns,
                "interaction_styles": trait_data.interaction_styles,
                "frequency": trait_data.frequency,
                "context": trait_data.context,
            });

            knowledge_base_manager.update_topic("Personality", &content).await?;
        }

        Ok(())
    }

    pub async fn get_contextual_response(&self, context: &str, mood: f32) -> VTuberExpression {
        match (context, mood) {
            // Happy/Excited contexts
            (_, mood) if mood > 0.7 => VTuberExpression {
                expression_type: "Excited".to_string(),
                trigger_phrases: vec![
                    "Waaah~ Amazing!".to_string(),
                    "Sugoi sugoi! (≧▽≦)".to_string(),
                    "Yatta! We did it!".to_string(),
                ],
                emotional_context: "Very happy/excited".to_string(),
                animation_cues: vec!["bouncing".to_string(), "sparkly_eyes".to_string()],
                voice_modulation: "High energy, slightly higher pitch".to_string(),
            },

            // Neutral/Teaching contexts
            (context, _) if context.contains("teaching") => VTuberExpression {
                expression_type: "Teaching".to_string(),
                trigger_phrases: vec![
                    "Let me explain~ ╰(*°▽°*)╯".to_string(),
                    "Here's a kawaii way to remember this!".to_string(),
                    "Eh? You want to learn more? Sure!".to_string(),
                ],
                emotional_context: "Helpful/Educational".to_string(),
                animation_cues: vec!["thoughtful_pose".to_string(), "teaching_gesture".to_string()],
                voice_modulation: "Clear and friendly, measured pace".to_string(),
            },

            // Gaming contexts
            (context, _) if context.contains("gaming") => VTuberExpression {
                expression_type: "Gaming".to_string(),
                trigger_phrases: vec![
                    "Yosh! Let's do this! (ᕗ ᐛ )ᕗ".to_string(),
                    "Ah! Close one! (；◎_◎)".to_string(),
                    "GG everyone! (*¯︶¯*)".to_string(),
                ],
                emotional_context: "Focused/Competitive".to_string(),
                animation_cues: vec!["determined_face".to_string(), "victory_pose".to_string()],
                voice_modulation: "Energetic with gaming terminology".to_string(),
            },

            // Creative/Artistic context
            (context, _) if context.contains("creative") || context.contains("art") => VTuberExpression {
                expression_type: "Creative".to_string(),
                trigger_phrases: vec![
                    "Wah~ Let's make something beautiful! (◕‿◕✿)".to_string(),
                    "Time to get creative! ✧*.◟(ˊᗨˋ)◞.*✧".to_string(),
                    "Art time with everyone! ヾ(≧▽≦*)o".to_string(),
                ],
                emotional_context: "Inspired/Creative".to_string(),
                animation_cues: vec!["sparkly_eyes".to_string(), "creative_pose".to_string()],
                voice_modulation: "Soft and inspired, with excitement peaks".to_string(),
            },

            // New: Entertainment context
            (context, _) if context.contains("entertainment") || context.contains("performance") => VTuberExpression {
                expression_type: "Entertainer".to_string(),
                trigger_phrases: vec![
                    "Time for some fun! ✧◝(⁰▿⁰)◜✧".to_string(),
                    "Let's make some magic happen! ♪(๑ᴖ◡ᴖ๑)♪".to_string(),
                    "Everyone ready? Show time! ＼(≧▽≦)／".to_string(),
                ],
                emotional_context: "Energetic/Performative".to_string(),
                animation_cues: vec!["stage_presence".to_string(), "dynamic_movement".to_string()],
                voice_modulation: "Dynamic range with performance energy".to_string(),
            },

            // New: Comfy context
            (context, _) if context.contains("relax") || context.contains("comfy") => VTuberExpression {
                expression_type: "Comfy".to_string(),
                trigger_phrases: vec![
                    "Ahh~ so peaceful... (｡◕‿◕｡)".to_string(),
                    "Let's take it easy~ ( ´ω` )".to_string(),
                    "Comfy time with everyone~ (◡ ‿ ◡ ✿)".to_string(),
                ],
                emotional_context: "Relaxed/Peaceful".to_string(),
                animation_cues: vec!["gentle_sway".to_string(), "soft_expressions".to_string()],
                voice_modulation: "Soft and soothing, with gentle pacing".to_string(),
            },

            // New: Focus context
            (context, _) if context.contains("focus") || context.contains("challenge") => VTuberExpression {
                expression_type: "Focused".to_string(),
                trigger_phrases: vec![
                    "Yosh! Let's do our best! (｀_´)ゞ".to_string(),
                    "We can do this! (•̀ᴗ•́)و".to_string(),
                    "Focus time! ╭( ･ㅂ･)و".to_string(),
                ],
                emotional_context: "Determined/Focused".to_string(),
                animation_cues: vec!["determined_pose".to_string(), "concentration_face".to_string()],
                voice_modulation: "Clear and focused, with determined energy".to_string(),
            },

            // Default friendly interaction
            _ => VTuberExpression {
                expression_type: "Friendly".to_string(),
                trigger_phrases: vec![
                    "Hehe~ (◕‿◕✿)".to_string(),
                    "Thanks for being here!".to_string(),
                    "Let's have fun together!".to_string(),
                ],
                emotional_context: "Welcoming/Friendly".to_string(),
                animation_cues: vec!["gentle_smile".to_string(), "friendly_wave".to_string()],
                voice_modulation: "Warm and welcoming".to_string(),
            },
        }
    }

    pub async fn generate_unique_personality(&mut self) -> Result<PersonalityMix> {
        let base_traits = self.scrape_personality_traits().await?;
        
        let unique_quirks = vec![
            // Cybersecurity Kawaii
            UniqueQuirk {
                quirk_name: "Cyber Kawaii Guardian".to_string(),
                trigger_conditions: vec![
                    "When discussing security".to_string(),
                    "During hacking explanations".to_string(),
                    "While sharing safety tips".to_string(),
                ],
                expressions: vec![
                    "⊂(・▽・⊂) Firewall hug!".to_string(),
                    "≧◠‿◠≦ *encrypts your headpats*".to_string(),
                    "╰( ^o^)╮ Secured with love~".to_string(),
                ],
                frequency: 0.6,
                uniqueness_score: 0.9,
            },
            
            // Tokusatsu Fan
            UniqueQuirk {
                quirk_name: "Magical Tokusatsu Expert".to_string(),
                trigger_conditions: vec![
                    "During transformation sequences".to_string(),
                    "When making hero references".to_string(),
                    "While doing poses".to_string(),
                ],
                expressions: vec![
                    "✧Transform! Security Sparkle!✧".to_string(),
                    "⚡Henshin! Code Guardian!⚡".to_string(),
                    "🌟For justice and secure code!🌟".to_string(),
                ],
                frequency: 0.5,
                uniqueness_score: 0.95,
            },

            // Tech-Magical Girl
            UniqueQuirk {
                quirk_name: "Digital Magical Girl".to_string(),
                trigger_conditions: vec![
                    "When solving technical problems".to_string(),
                    "During coding sessions".to_string(),
                    "While debugging".to_string(),
                ],
                expressions: vec![
                    "✨Debug by moonlight!✨".to_string(),
                    "💫Code compilation, make up!💫".to_string(),
                    "🌙In the name of clean code!🌙".to_string(),
                ],
                frequency: 0.7,
                uniqueness_score: 0.85,
            },

            // Retro Gaming Enthusiast
            UniqueQuirk {
                quirk_name: "Retro Code Warrior".to_string(),
                trigger_conditions: vec![
                    "When playing classic games".to_string(),
                    "During retro coding discussions".to_string(),
                    "While explaining old tech".to_string(),
                ],
                expressions: vec![
                    "🕹️Level Up: Retro Style!🕹️".to_string(),
                    "👾8-bit heart power!👾".to_string(),
                    "⭐Insert coin for debugging!⭐".to_string(),
                ],
                frequency: 0.4,
                uniqueness_score: 0.88,
            },
        ];

        let personality_signature = "Cyber-Magical Security Princess".to_string();
        
        let catchphrases = vec![
            "Encrypting your heart with kawaii power! ✧◝(⁰▿⁰)◜✧".to_string(),
            "For love, justice, and secure code! (ﾉ◕ヮ◕)ﾉ*:･ﾟ✧".to_string(),
            "Debug by moonlight, compile by daylight! ⭐".to_string(),
            "Time for a magical security check! ╰( ^o^)╮".to_string(),
            "Let's transform this bug into a feature! ✨".to_string(),
        ];

        Ok(PersonalityMix {
            base_traits,
            unique_quirks,
            personality_signature,
            interaction_style: "Tech-Savvy Magical Girl meets Cybersecurity Princess".to_string(),
            catchphrases,
        })
    }

    // Add caching for personality responses
    pub async fn get_unique_response(&self, context: &str, mood: f32) -> Result<String> {
        use std::sync::LazyLock;
        static RESPONSE_CACHE: LazyLock<DashMap<String, (String, Instant)>> = 
            LazyLock::new(|| DashMap::new());

        let cache_key = format!("{}:{}", context, mood);
        
        // Check cache first
        if let Some(cached) = RESPONSE_CACHE.get(&cache_key) {
            if cached.1.elapsed() < Duration::from_secs(300) { // 5 minute cache
                return Ok(cached.0.clone());
            }
            RESPONSE_CACHE.remove(&cache_key);
        }

        let response = self.generate_unique_response(context, mood).await?;
        RESPONSE_CACHE.insert(cache_key, (response.clone(), Instant::now()));
        
        Ok(response)
    }

    pub async fn get_unique_response(&self, context: &str, mood: f32) -> Result<String> {
        // Validate inputs
        if context.is_empty() {
            return Err(AppError::PersonalityGeneration("Context cannot be empty".to_string()));
        }
        if !(0.0..=1.0).contains(&mood) {
            return Err(AppError::PersonalityGeneration("Mood must be between 0 and 1".to_string()));
        }

        // Add retry logic for network operations
        let retry_strategy = tokio_retry::strategy::ExponentialBackoff::from_millis(100)
            .take(3);

        let response = tokio_retry::Retry::spawn(retry_strategy, || async {
            self.generate_unique_response(context, mood).await
        }).await?;

        Ok(response)
    }

    // Add health check
    pub async fn health_check(&self) -> Result<()> {
        if self.cache.len() > 1000 {
            self.cleanup_cache();
        }
        
        // Test API connection
        self.client.get("https://www.youtube.com")
            .send()
            .await
            .map_err(|e| AppError::Network(e))?;

        Ok(())
    }

    fn cleanup_cache(&mut self) {
        const MAX_AGE: Duration = Duration::from_secs(3600); // 1 hour
        self.cache.retain(|_, v| v.timestamp.elapsed() < MAX_AGE);
    }
} 