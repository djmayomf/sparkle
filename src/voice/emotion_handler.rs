use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::mpsc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Emotion {
    Happy,
    Excited,
    Focused,
    Playful,
    Sassy,
    Determined,
    Kawaii,
    Energetic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionState {
    pub current_emotion: Emotion,
    pub intensity: f32,  // 0.0 to 1.0
    pub duration: Option<std::time::Duration>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct EmotionHandler {
    current_state: EmotionState,
    emotion_phrases: HashMap<Emotion, Vec<String>>,
    emotion_emotes: HashMap<Emotion, Vec<String>>,
    voice_tx: mpsc::Sender<String>,
    last_toggle: std::time::Instant,
}

impl EmotionHandler {
    pub fn new(voice_tx: mpsc::Sender<String>) -> Self {
        Self {
            current_state: EmotionState {
                current_emotion: Emotion::Happy,
                intensity: 0.7,
                duration: None,
                timestamp: chrono::Utc::now(),
            },
            emotion_phrases: Self::init_emotion_phrases(),
            emotion_emotes: Self::init_emotion_emotes(),
            voice_tx,
            last_toggle: std::time::Instant::now(),
        }
    }

    fn init_emotion_phrases() -> HashMap<Emotion, Vec<String>> {
        let mut phrases = HashMap::new();
        
        phrases.insert(Emotion::Happy, vec![
            "*bounces happily* ".to_string(),
            "*giggles* ".to_string(),
            "*smiles brightly* ".to_string(),
        ]);

        phrases.insert(Emotion::Excited, vec![
            "*jumps excitedly* ".to_string(),
            "*sparkles with energy* ".to_string(),
            "*vibrates with excitement* ".to_string(),
        ]);

        phrases.insert(Emotion::Focused, vec![
            "*adjusts glasses* ".to_string(),
            "*concentrates intensely* ".to_string(),
            "*enters hacker mode* ".to_string(),
        ]);

        phrases.insert(Emotion::Playful, vec![
            "*spins around* ".to_string(),
            "*does a little dance* ".to_string(),
            "*strikes a cute pose* ".to_string(),
        ]);

        phrases.insert(Emotion::Sassy, vec![
            "*flips hair* ".to_string(),
            "*smirks knowingly* ".to_string(),
            "*raises eyebrow* ".to_string(),
        ]);

        phrases.insert(Emotion::Determined, vec![
            "*cracks knuckles* ".to_string(),
            "*rolls up sleeves* ".to_string(),
            "*game face on* ".to_string(),
        ]);

        phrases.insert(Emotion::Kawaii, vec![
            "*tilts head cutely* ".to_string(),
            "*makes heart with hands* ".to_string(),
            "*radiates kawaii energy* ".to_string(),
        ]);

        phrases.insert(Emotion::Energetic, vec![
            "*bounces off walls* ".to_string(),
            "*zooms around* ".to_string(),
            "*overflows with energy* ".to_string(),
        ]);

        phrases
    }

    fn init_emotion_emotes() -> HashMap<Emotion, Vec<String>> {
        let mut emotes = HashMap::new();
        
        emotes.insert(Emotion::Happy, vec!["(◕‿◕✿)".to_string(), "(*≧ω≦*)".to_string()]);
        emotes.insert(Emotion::Excited, vec!["(ﾉ◕ヮ◕)ﾉ*:･ﾟ✧".to_string(), "(★^O^★)".to_string()]);
        emotes.insert(Emotion::Focused, vec!["(｀_´)ゞ".to_string(), "(•̀ᴗ•́)و".to_string()]);
        emotes.insert(Emotion::Playful, vec!["(｡♥‿♥｡)".to_string(), "(◠‿◠✿)".to_string()]);
        emotes.insert(Emotion::Sassy, vec!["(︶｡︶✽)".to_string(), "( ˘ ³˘)♥".to_string()]);
        emotes.insert(Emotion::Determined, vec!["(ง •̀_•́)ง".to_string(), "( •̀ᄇ• ́)ﻭ✧".to_string()]);
        emotes.insert(Emotion::Kawaii, vec!["(◕‿◕✿)".to_string(), "(｡◕‿◕｡)".to_string()]);
        emotes.insert(Emotion::Energetic, vec!["⚡(◕‿◕)⚡".to_string(), "✨(◕‿◕)✨".to_string()]);
        
        emotes
    }

    pub async fn toggle_emotion(&mut self, emotion: Emotion) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure minimum time between toggles (50ms)
        if self.last_toggle.elapsed() < std::time::Duration::from_millis(50) {
            return Ok(());
        }

        self.current_state = EmotionState {
            current_emotion: emotion.clone(),
            intensity: 1.0,  // Full intensity on toggle
            duration: None,
            timestamp: chrono::Utc::now(),
        };

        // Send immediate feedback to voice system
        if let Some(phrase) = self.get_emotion_phrase(&emotion) {
            self.voice_tx.send(phrase).await?;
        }

        self.last_toggle = std::time::Instant::now();
        Ok(())
    }

    pub fn apply_emotion(&self, text: &str) -> String {
        let emotion = &self.current_state.current_emotion;
        let intensity = self.current_state.intensity;

        // Get random phrase and emote for current emotion
        let phrase = self.get_random_phrase(emotion);
        let emote = self.get_random_emote(emotion);

        // Apply emotion based on intensity
        if intensity > 0.8 {
            format!("{}{} {} {}", phrase, text, emote, emote)
        } else if intensity > 0.5 {
            format!("{}{} {}", phrase, text, emote)
        } else {
            format!("{} {}", text, emote)
        }
    }

    fn get_random_phrase(&self, emotion: &Emotion) -> String {
        if let Some(phrases) = self.emotion_phrases.get(emotion) {
            phrases[fastrand::usize(..phrases.len())].clone()
        } else {
            String::new()
        }
    }

    fn get_random_emote(&self, emotion: &Emotion) -> String {
        if let Some(emotes) = self.emotion_emotes.get(emotion) {
            emotes[fastrand::usize(..emotes.len())].clone()
        } else {
            "✨".to_string()
        }
    }

    pub fn get_emotion_phrase(&self, emotion: &Emotion) -> Option<String> {
        self.emotion_phrases.get(emotion)
            .and_then(|phrases| phrases.get(fastrand::usize(..phrases.len())))
            .cloned()
    }

    pub fn is_emotion_active(&self, emotion: &Emotion) -> bool {
        self.current_state.current_emotion == *emotion
    }

    pub fn get_current_emotion(&self) -> &Emotion {
        &self.current_state.current_emotion
    }
} 