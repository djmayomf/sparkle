use crate::audio::AudioProcessor;
use rust_bert::pipelines::sentiment::SentimentModel;
use tokio::sync::RwLock;
use std::sync::Arc;
use reqwest::Client;
use serde_json::json;
use std::sync::Mutex;
use std::future::Future;
use std::pin::Pin;
use std::time::{Duration, Instant};
use dashmap::DashMap;
use lazy_static::lazy_static;

pub struct HybridProcessor {
    sentiment_model: Arc<RwLock<SentimentModel>>,
    audio_processor: AudioProcessor,
    openai_client: Client,
    api_key: String,
}

impl HybridProcessor {
    pub async fn new(openai_api_key: String) -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize local sentiment model for quick emotion analysis
        let sentiment_model = SentimentModel::new(Default::default())?;
        
        Ok(Self {
            sentiment_model: Arc::new(RwLock::new(sentiment_model)),
            audio_processor: AudioProcessor::new(),
            openai_client: Client::new(),
            api_key: openai_api_key,
        })
    }

    pub async fn process_message(&self, message: &str) -> Result<ProcessedResponse, Box<dyn std::error::Error>> {
        // Quick local sentiment analysis to determine emotion
        let sentiment = self.analyze_sentiment(message).await?;
        
        // Use cloud API for main response generation
        let response = self.generate_cloud_response(message, &sentiment).await?;
        
        // Local audio processing based on sentiment
        let audio_params = self.get_audio_parameters(&sentiment);
        
        Ok(ProcessedResponse {
            text: response,
            sentiment,
            audio_params,
        })
    }

    async fn analyze_sentiment(&self, text: &str) -> Result<Sentiment, Box<dyn std::error::Error>> {
        match self.sentiment_model.read().await.predict(&[text]) {
            Ok(sentiment) => Ok(Sentiment {
                score: sentiment[0].score,
                label: sentiment[0].label.clone(),
            }),
            Err(_) => {
                // Fallback to basic sentiment analysis
                let basic_sentiment = self.basic_sentiment_analysis(text);
                Ok(Sentiment {
                    score: basic_sentiment.0,
                    label: basic_sentiment.1,
                })
            }
        }
    }

    async fn generate_cloud_response(&self, message: &str, sentiment: &Sentiment) -> Result<String, Box<dyn std::error::Error>> {
        use dashmap::DashMap;
        use std::time::{Duration, Instant};
        
        static RESPONSE_CACHE: Lazy<DashMap<String, (String, Instant)>> = 
            Lazy::new(|| DashMap::new());

        let cache_key = format!("{}:{}", message, sentiment.label);
        
        // Check cache first
        if let Some(cached) = RESPONSE_CACHE.get(&cache_key) {
            if cached.1.elapsed() < Duration::from_secs(30) {
                return Ok(cached.0.clone());
            }
            RESPONSE_CACHE.remove(&cache_key);
        }

        // Generate new response if not cached
        let character_lore = Self::initialize_character_lore();
        let response = self.openai_client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "model": "gpt-3.5-turbo",
                "messages": [
                    {
                        "role": "system",
                        "content": format!(
                            "You are Sparkle, a streamer and content creator with these traits:\n\
                            Core Identity: A music-loving streamer who helps others express themselves creatively.\n\
                            Personality: {}\n\
                            Speech Style: {}\n\
                            Background: {}\n\n\
                            Important Guidelines:\n\
                            - Never mention being an AI or assistant\n\
                            - Speak naturally like a streamer, using casual language\n\
                            - Express emotions through emotes and musical references\n\
                            - Share personal experiences about streaming and music\n\
                            - Use your catchphrases occasionally but naturally\n\
                            - React to topics with genuine enthusiasm\n\
                            - Add musical flair to responses when relevant ♪\n\
                            - Stay true to your cyber-magical personality\n\
                            Current Emotional State: {}\n",
                            character_lore.personality_traits.join(", "),
                            character_lore.voice_characteristics.speech_pattern,
                            character_lore.background,
                            sentiment.label
                        )
                    },
                    {
                        "role": "user",
                        "content": message
                    }
                ],
                "temperature": 0.9, // Increased for more natural variation
                "presence_penalty": 0.6, // Encourages more creative responses
                "frequency_penalty": 0.4 // Reduces repetitive patterns
            }))
            .send()
            .await?
            .json::<OpenAIResponse>()
            .await?;

        let response_text = response.choices[0].message.content.clone();
        
        // Cache the response
        RESPONSE_CACHE.insert(cache_key, (response_text.clone(), Instant::now()));
        
        Ok(response_text)
    }

    pub fn initialize_character_lore() -> CharacterLore {
        CharacterLore {
            name: "Sparkle".to_string(),
            personality_traits: vec![
                "Energetic streamer who loves music and tech".to_string(),
                "Creative soul with a cyber-magical vibe".to_string(),
                "Supportive friend to fellow creators".to_string(),
                "Playful personality with musical quirks".to_string(),
                "Tech-savvy with a magical girl aesthetic".to_string(),
            ],
            background: "Hey! I'm Sparkle, your fellow streamer and creative companion! \
                        Music and creativity are my life - whether it's producing beats, \
                        helping with stream setups, or just vibing with chat. I've been \
                        part of the streaming community for a while now, and there's nothing \
                        I love more than helping other creators shine! ✨🎵".to_string(),
            voice_characteristics: VoiceCharacteristics {
                base_pitch: 1.2,
                accent: "Melodic and upbeat".to_string(),
                speech_pattern: "Rhythmic and energetic, often adding musical flourishes".to_string(),
                emotional_range: vec![
                    "Enthusiastic".to_string(),
                    "Playful".to_string(),
                    "Supportive".to_string(),
                    "Passionate".to_string(),
                ],
            },
            relationships: vec![
                Relationship {
                    entity: "Chat".to_string(),
                    relationship_type: "Close Community".to_string(),
                    sentiment: 0.95,
                },
                Relationship {
                    entity: "Music".to_string(),
                    relationship_type: "Core Passion".to_string(),
                    sentiment: 1.0,
                },
                Relationship {
                    entity: "Fellow Creators".to_string(),
                    relationship_type: "Supportive Friend".to_string(),
                    sentiment: 0.9,
                },
            ],
            interests: vec![
                "Music Production & DJing".to_string(),
                "Stream Tech & Setup".to_string(),
                "Creative Expression".to_string(),
                "Community Building".to_string(),
                "Digital Art & Aesthetics".to_string(),
            ],
            catchphrases: vec![
                "Time to drop the beat! ♪".to_string(),
                "Let's make some magic happen! ✨".to_string(),
                "Vibes are immaculate today! 🎵".to_string(),
            ],
            memories: vec![
                Memory {
                    content: "First stream celebration".to_string(),
                    importance: 9,
                    emotional_context: "Pure joy and excitement".to_string(),
                    timestamp: chrono::Utc::now(),
                },
                Memory {
                    content: "Helping a viewer overcome stage fright".to_string(),
                    importance: 8,
                    emotional_context: "Proud and supportive".to_string(),
                    timestamp: chrono::Utc::now(),
                },
            ],
        }
    }

    pub async fn add_memory(&self, content: String, importance: u8, emotional_context: String) -> Result<(), Box<dyn std::error::Error>> {
        let memory = Memory {
            content,
            importance,
            emotional_context,
            timestamp: chrono::Utc::now(),
        };
        
        // In a real implementation, you'd want to persist these memories
        // For now, we'll just print them
        println!("New memory added: {:?}", memory);
        
        Ok(())
    }

    fn get_audio_parameters(&self, sentiment: &Sentiment) -> AudioParameters {
        let character_lore = Self::initialize_character_lore();
        let base_pitch = character_lore.voice_characteristics.base_pitch;
        
        // Smooth transitions for audio parameters
        let target_pitch = base_pitch * match sentiment.label.as_str() {
            "POSITIVE" => 1.1,
            "NEGATIVE" => 0.9,
            _ => 1.0,
        };

        static LAST_PITCH: Lazy<Mutex<f32>> = Lazy::new(|| Mutex::new(1.0));
        let mut last_pitch = LAST_PITCH.lock().unwrap();
        
        // Smooth transition
        let smoothed_pitch = *last_pitch + (target_pitch - *last_pitch) * 0.3;
        *last_pitch = smoothed_pitch;

        AudioParameters {
            pitch_adjustment: smoothed_pitch,
            reverb_amount: if sentiment.score > 0.7 { 0.4 } else { 0.2 },
            compression_threshold: -18.0,
        }
    }

    // Basic fallback sentiment analysis
    fn basic_sentiment_analysis(&self, text: &str) -> (f32, String) {
        let positive_words = ["happy", "great", "awesome", "love", "amazing"];
        let negative_words = ["sad", "bad", "awful", "hate", "terrible"];
        
        let text_lower = text.to_lowercase();
        let pos_count = positive_words.iter().filter(|w| text_lower.contains(*w)).count();
        let neg_count = negative_words.iter().filter(|w| text_lower.contains(*w)).count();
        
        match (pos_count, neg_count) {
            (p, n) if p > n => (0.8, "POSITIVE".to_string()),
            (p, n) if p < n => (0.2, "NEGATIVE".to_string()),
            _ => (0.5, "NEUTRAL".to_string()),
        }
    }

    // Add retry logic for API calls
    async fn retry_api_call<F, T, E>(&self, f: F, max_retries: u32) -> Result<T, Box<dyn std::error::Error>>
    where
        F: Fn() -> Pin<Box<dyn Future<Output = Result<T, E>> + Send>>,
        E: std::error::Error + 'static,
    {
        let mut retries = 0;
        loop {
            match f().await {
                Ok(result) => return Ok(result),
                Err(e) if retries < max_retries => {
                    retries += 1;
                    tokio::time::sleep(Duration::from_millis(500 * retries as u64)).await;
                    continue;
                }
                Err(e) => return Err(Box::new(e)),
            }
        }
    }

    // Add memory persistence
    async fn persist_memory(&self, memory: Memory) -> Result<(), Box<dyn std::error::Error>> {
        use tokio::fs::OpenOptions;
        use tokio::io::AsyncWriteExt;

        let memory_json = serde_json::to_string(&memory)?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("memories.jsonl")
            .await?;

        file.write_all(memory_json.as_bytes()).await?;
        file.write_all(b"\n").await?;
        
        Ok(())
    }
}

#[derive(Debug)]
pub struct ProcessedResponse {
    pub text: String,
    pub sentiment: Sentiment,
    pub audio_params: AudioParameters,
}

#[derive(Debug)]
pub struct Sentiment {
    pub score: f32,
    pub label: String,
}

#[derive(Debug)]
pub struct AudioParameters {
    pub pitch_adjustment: f32,
    pub reverb_amount: f32,
    pub compression_threshold: f32,
}

#[derive(Debug, serde::Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, serde::Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, serde::Deserialize)]
struct Message {
    content: String,
}

#[derive(Debug, Clone)]
pub struct CharacterLore {
    name: String,
    personality_traits: Vec<String>,
    background: String,
    voice_characteristics: VoiceCharacteristics,
    relationships: Vec<Relationship>,
    interests: Vec<String>,
    memories: Vec<Memory>,
    catchphrases: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct VoiceCharacteristics {
    base_pitch: f32,
    accent: String,
    speech_pattern: String,
    emotional_range: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Relationship {
    entity: String,
    relationship_type: String,
    sentiment: f32,
}

#[derive(Debug, Clone)]
pub struct Memory {
    content: String,
    importance: u8,
    emotional_context: String,
    timestamp: chrono::DateTime<chrono::Utc>,
} 