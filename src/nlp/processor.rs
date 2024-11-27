use rust_bert::pipelines::sentiment::SentimentModel;
use crate::database::connection::KnowledgeBaseManager;
use crate::scrapers::youtube_personality_scraper::YouTubePersonalityScraper;
use std::sync::Arc;
use std::time::{Duration, Instant};
use dashmap::DashMap;

pub struct NLPProcessor {
    sentiment_model: Arc<SentimentModel>,
    personality_scraper: Arc<YouTubePersonalityScraper>,
    response_cache: Arc<DashMap<String, (String, Instant)>>,
}

impl NLPProcessor {
    pub async fn new(pool: sqlx::PgPool) -> Self {
        let sentiment_model = Arc::new(SentimentModel::new(Default::default()).unwrap());
        let personality_scraper = Arc::new(YouTubePersonalityScraper::new("playlist_id"));
        
        Self {
            sentiment_model,
            personality_scraper,
            response_cache: Arc::new(DashMap::new()),
        }
    }

    pub async fn fetch_knowledge(&self, topic: &str) -> Result<String, Box<dyn std::error::Error>> {
        let knowledge_base_manager = KnowledgeBaseManager::new(self.pool.clone()).await;
        let content = knowledge_base_manager.get_topic(topic).await?;
        Ok(content.to_string())
    }

    pub async fn process_message(&self, message: &str) -> String {
        // Check if the message contains a request for knowledge
        if message.contains("tell me about") {
            let topic = message.split("tell me about").nth(1).unwrap_or("").trim();
            if let Ok(content) = self.fetch_knowledge(topic).await {
                return format!("Here's what I know about {}: {}", topic, content);
            }
        }
        
        // Existing sentiment analysis and response generation
        let sentiment = self.sentiment_model.predict(&[message]);
        match sentiment[0] {
            s if s > 0.7 => "I'm happy you're excited! (*≧ω≦*)".to_string(),
            s if s < 0.3 => "Don't worry, things will get better! (｀・ω・´)".to_string(),
            _ => "Thanks for chatting with me! (◕‿◕✿)".to_string(),
        }
    }

    // Function to update response model based on feedback
    pub fn update_response_model(feedback: &str) {
        // Process feedback and adjust model parameters
        adjust_model_parameters(feedback);
    }

    // Function to analyze sentiment
    pub fn analyze_sentiment(message: &str) -> Sentiment {
        // Use NLP library to determine sentiment
        let sentiment = nlp_library::analyze(message);
        sentiment
    }

    // Adjust response based on sentiment
    pub fn respond_with_emotion(sentiment: Sentiment) -> String {
        match sentiment {
            Sentiment::Positive => "I'm glad you're enjoying the stream! 😊",
            Sentiment::Negative => "I'm sorry to hear that. Let me know how I can help. 😟",
            Sentiment::Neutral => "Thanks for your input! Let's keep the fun going! 😄",
        }
    }

    pub async fn get_personality_response(&self, context: &str, sentiment: f32) -> String {
        let cache_key = format!("{}:{}", context, sentiment);
        
        // Check cache
        if let Some(cached) = self.response_cache.get(&cache_key) {
            if cached.1.elapsed() < Duration::from_secs(60) {
                return cached.0.clone();
            }
            self.response_cache.remove(&cache_key);
        }

        let response = self.personality_scraper
            .get_unique_response(context, sentiment)
            .await
            .unwrap_or_else(|_| "Hehe~ (◕‿◕✿)".to_string());

        self.response_cache.insert(cache_key, (response.clone(), Instant::now()));
        response
    }

    pub async fn get_unique_personality_response(&self, context: &str, sentiment: f32) -> String {
        let personality_scraper = YouTubePersonalityScraper::new("PLJKnqkmW4RmTrorVKRwqY1ESDjLUfunyg");
        personality_scraper.get_unique_response(context, sentiment).await
    }
} 