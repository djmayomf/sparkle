use crate::audio::AudioProcessor;
use rust_bert::pipelines::sentiment::SentimentModel;
use tokio::sync::RwLock;
use std::sync::Arc;
use reqwest::Client;

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
        let model = self.sentiment_model.read().await;
        let sentiment = model.predict(&[text])?;
        
        Ok(Sentiment {
            score: sentiment[0].score,
            label: sentiment[0].label.clone(),
        })
    }

    async fn generate_cloud_response(&self, message: &str, sentiment: &Sentiment) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.openai_client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({
                "model": "gpt-3.5-turbo",
                "messages": [
                    {
                        "role": "system",
                        "content": "You are a helpful assistant with a focus on natural conversation."
                    },
                    {
                        "role": "user",
                        "content": message
                    }
                ],
                "temperature": 0.7
            }))
            .send()
            .await?
            .json::<OpenAIResponse>()
            .await?;

        Ok(response.choices[0].message.content.clone())
    }

    fn get_audio_parameters(&self, sentiment: &Sentiment) -> AudioParameters {
        // Adjust audio parameters based on sentiment
        AudioParameters {
            pitch_adjustment: match sentiment.label.as_str() {
                "POSITIVE" => 1.1,
                "NEGATIVE" => 0.9,
                _ => 1.0,
            },
            reverb_amount: if sentiment.score > 0.7 { 0.4 } else { 0.2 },
            compression_threshold: -18.0,
        }
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