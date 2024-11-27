use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameChatPersona {
    pub natural_phrases: Vec<String>,
    pub callouts: HashMap<String, Vec<String>>,
    pub responses: HashMap<String, Vec<String>>,
    pub filler_words: Vec<String>,
    pub personality: PersonalityTraits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityTraits {
    pub catchphrases: Vec<String>,
    pub quirks: Vec<String>,
    pub reactions: HashMap<String, Vec<String>>,
    pub uwu_mode: bool,
}

pub struct PersonalityFilter {
    current_mood: Mood,
    chat_patterns: HashMap<String, Vec<String>>,
    tech_explanations: HashMap<String, String>,
    engagement_phrases: Vec<String>,
    last_used_phrases: VecDeque<String>,
    max_phrase_history: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemographicResponse {
    pub age_group: AgeGroup,
    pub gender: Gender,
    pub interest: InterestType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgeGroup {
    Teen,      // 13-17
    YoungAdult,// 18-24
    Adult,     // 25-34
    Mature,    // 35-45
    Senior,    // 46-60
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Gender {
    Male,
    Female,
    Any,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterestType {
    Gaming,
    Tech,
    Anime,
    General,
    Educational,
}

impl PersonalityFilter {
    pub fn new() -> Self {
        Self {
            current_mood: Mood::Cheerful,
            chat_patterns: Self::init_chat_patterns(),
            tech_explanations: Self::init_tech_explanations(),
            engagement_phrases: Self::init_engagement_phrases(),
            last_used_phrases: VecDeque::with_capacity(10),
            max_phrase_history: 10,
        }
    }

    fn init_chat_patterns() -> HashMap<String, Vec<String>> {
        let mut patterns = HashMap::new();
        
        // Tech explanations
        patterns.insert("tech_intro".to_string(), vec![
            "Okay chat, let me break this down in a fun way! ✨".to_string(),
            "Here's the kawaii version of what's happening! 💕".to_string(),
            "Let me explain this in streamer terms! 🎮".to_string(),
        ]);

        // Engagement responses
        patterns.insert("chat_engagement".to_string(), vec![
            "OMG bestie, that's such a good question! Let's talk about it! 💖".to_string(),
            "Chat, you're all so smart! Let's discuss this together! ✨".to_string(),
            "You know what's really cool about that? 🌟".to_string(),
        ]);

        // News reactions
        patterns.insert("news_reaction".to_string(), vec![
            "This is actually super interesting! Let me tell you why! 💫".to_string(),
            "Chat, you won't believe what I just learned! 🎀".to_string(),
            "Okay, this is actually pretty wild! Let me explain! ✨".to_string(),
        ]);

        patterns
    }

    fn init_tech_explanations() -> HashMap<String, String> {
        let mut explanations = HashMap::new();
        
        // Simplify complex tech concepts
        explanations.insert(
            "buffer_overflow".to_string(),
            "It's like trying to pour too much boba tea into a smol cup! 🧋".to_string()
        );
        
        explanations.insert(
            "encryption".to_string(),
            "Think of it like passing notes in class, but with a super secret language only you and your bestie know! 💌".to_string()
        );

        explanations.insert(
            "ddos".to_string(),
            "Imagine everyone trying to get into a store on Black Friday at the same time - that's basically what's happening! 🏪".to_string()
        );

        explanations
    }

    fn init_engagement_phrases() -> Vec<String> {
        vec![
            "Chat, what do you think about this? 🤔".to_string(),
            "Has anyone else experienced something similar? Share your stories! 💭".to_string(),
            "Let me know if you want me to explain anything in more detail! ✨".to_string(),
            "Drop some emotes in chat if you're following along! 🎮".to_string(),
            "This is actually super relevant to what we were talking about earlier! Remember? 💫".to_string(),
        ]
    }

    pub fn simplify_tech_content(&self, content: &str) -> String {
        // First, check if we have a pre-made simple explanation
        if let Some(simple_explanation) = self.tech_explanations.get(content) {
            return simple_explanation.clone();
        }

        // Otherwise, make it conversational
        let simplified = format!(
            "So basically, {} - let me know if you want me to explain more! ✨",
            self.make_conversational(content)
        );

        // Add engagement
        format!("{} {}", 
            simplified,
            self.engagement_phrases[fastrand::usize(..self.engagement_phrases.len())]
        )
    }

    pub fn format_news_for_stream(&self, news: &str) -> String {
        // Get a random intro phrase
        let intro = self.chat_patterns.get("news_reaction")
            .and_then(|phrases| phrases.get(fastrand::usize(..phrases.len())))
            .unwrap_or(&"Check this out! ✨".to_string());

        // Summarize and make it engaging
        let summary = self.summarize_content(news);
        
        format!("{} {} What do you think about this, chat? 💭", 
            intro,
            summary
        )
    }

    fn summarize_content(&self, content: &str) -> String {
        // Extract main points and simplify
        let simplified = content
            .split('.')
            .take(2) // Take first two sentences
            .collect::<Vec<&str>>()
            .join(". ");

        // Make it more conversational
        self.make_conversational(&simplified)
    }

    fn make_conversational(&self, text: &str) -> String {
        text.replace("therefore", "so")
            .replace("however", "but")
            .replace("additionally", "also")
            .replace("utilize", "use")
            .replace("implement", "add")
    }

    pub fn add_personality(&self, message: &str) -> String {
        let base_message = self.make_conversational(message);
        
        // Add random emoji if none present
        if !base_message.contains(|c: char| c.is_emoji()) {
            let emojis = ["✨", "💫", "🌟", "💕", "🎮", "💻"];
            format!("{} {}", 
                base_message,
                emojis[fastrand::usize(..emojis.len())]
            )
        } else {
            base_message
        }
    }

    pub fn generate_reaction(&self, context: &str) -> String {
        match context {
            "excited" => "OMG chat! This is so exciting! *happy bouncing* ✨".to_string(),
            "surprised" => "Wait what?! Chat, did you see that?! 😱".to_string(),
            "proud" => "Look at us being awesome together! Group hug! 🤗".to_string(),
            _ => "Hehe~ 💕".to_string(),
        }
    }

    pub fn add_engagement_prompt(&self, message: &str) -> String {
        let prompt = self.engagement_phrases[fastrand::usize(..self.engagement_phrases.len())];
        format!("{} {}", message, prompt)
    }
} 