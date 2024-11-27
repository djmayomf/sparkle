use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::time::Duration;
use youtube_dl::{YoutubeDl, YoutubeDlOutput};
use super::bias_detector::BiasDetector;
use super::opinion_former::OpinionFormer;

#[derive(Debug, Serialize, Deserialize)]
pub struct TechKnowledgeBase {
    pub channels: HashMap<String, ChannelData>,
    pub topics: HashMap<String, TopicData>,
    pub products: HashMap<String, ProductData>,
    pub reviews: Vec<TechReview>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub opinion_system: OpinionSystem,
    pub bias_detector: BiasDetector,
    pub sponsorship_tracker: SponsorshipTracker,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelData {
    pub channel_id: String,
    pub playlists: Vec<PlaylistInfo>,
    pub video_count: u32,
    pub specialties: Vec<String>,
    pub content_style: ContentStyle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistInfo {
    pub id: String,
    pub title: String,
    pub video_count: u32,
    pub category: TechCategory,
    pub topics: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoInfo {
    pub id: String,
    pub title: String,
    pub category: TechCategory,
    pub product_name: Option<String>,
    pub transcript: String,
    pub topics: Vec<String>,
    pub publish_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechReview {
    pub product_name: String,
    pub rating: f32,
    pub pros: Vec<String>,
    pub cons: Vec<String>,
    pub key_features: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TechCategory {
    Review,
    Comparison,
    News,
    Analysis,
    FirstImpressions,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContentStyle {
    TechEducation,
    ProductReview,
    DetailedAnalysis,
    EntertainmentFocus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicData {
    pub name: String,
    pub videos: Vec<String>,
    pub related_topics: Vec<String>,
    pub expertise_level: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductData {
    pub name: String,
    pub category: String,
    pub reviews: Vec<String>,
    pub average_rating: f32,
    pub key_features: Vec<String>,
}

impl TechKnowledgeBase {
    pub async fn new() -> Self {
        let mut base = Self {
            channels: HashMap::new(),
            topics: HashMap::new(),
            products: HashMap::new(),
            reviews: Vec::new(),
            last_updated: chrono::Utc::now(),
            opinion_system: OpinionSystem {
                tech_opinions: HashMap::new(),
                opinion_factors: OpinionFactors::default(),
                confidence_levels: HashMap::new(),
                experience_data: ExperienceData::default(),
            },
            bias_detector: BiasDetector::new(),
            sponsorship_tracker: SponsorshipTracker {
                active_sponsorships: HashMap::new(),
                sponsorship_history: Vec::new(),
                disclosure_requirements: DisclosurePolicy::default(),
            },
        };

        // Initialize LTT data
        base.channels.insert(
            "LinusTechTips".to_string(),
            ChannelData {
                channel_id: "UCXuqSBlHAE6Xw-yeJA0Tunw".to_string(),
                playlists: Vec::new(),
                video_count: 0,
                specialties: vec![
                    "PC Hardware".to_string(),
                    "Tech Reviews".to_string(),
                    "DIY Projects".to_string(),
                ],
                content_style: ContentStyle::TechEducation,
            }
        );

        // Initialize MKBHD data
        base.channels.insert(
            "MKBHD".to_string(),
            ChannelData {
                channel_id: "UCBJycsmduvYEL83R_U4JriQ".to_string(),
                playlists: Vec::new(),
                video_count: 0,
                specialties: vec![
                    "Smartphones".to_string(),
                    "Premium Tech".to_string(),
                    "Future Tech".to_string(),
                ],
                content_style: ContentStyle::ProductReview,
            }
        );

        base
    }

    pub async fn form_opinion(&mut self, topic: &str) -> Result<FormedOpinion, String> {
        // Gather all available data
        let ltt_data = self.get_channel_data("LinusTechTips", topic).await?;
        let mkbhd_data = self.get_channel_data("MKBHD", topic).await?;
        
        // Check for sponsorships
        let is_sponsored = self.sponsorship_tracker.check_sponsorship(topic).await?;
        
        // Detect and filter bias
        let filtered_ltt = self.bias_detector.analyze_content(&ltt_data).await?;
        let filtered_mkbhd = self.bias_detector.analyze_content(&mkbhd_data).await?;
        
        // Form independent opinion
        let opinion = self.opinion_system.form_opinion(
            topic,
            vec![filtered_ltt, filtered_mkbhd],
            is_sponsored
        ).await?;
        
        Ok(opinion)
    }

    pub async fn express_opinion(&self, topic: &str) -> Result<String, String> {
        if let Some(opinion) = self.opinion_system.tech_opinions.get(topic) {
            // If sponsored, clearly disclose it
            if opinion.is_sponsored {
                return Ok(format!(
                    "Sponsored Opinion on {}: {}. [This is a sponsored opinion]",
                    topic,
                    self.format_opinion(opinion)
                ));
            }

            // Otherwise, express genuine opinion
            Ok(format!(
                "My thoughts on {}: {}. Based on technical analysis and {} confidence level.",
                topic,
                self.format_opinion(opinion),
                self.format_confidence(opinion.confidence_score)
            ))
        } else {
            Ok(format!("I haven't formed a complete opinion on {} yet.", topic))
        }
    }

    fn format_opinion(&self, opinion: &FormedOpinion) -> String {
        let mut formatted = String::new();
        
        // Add key points with reasoning
        for point in &opinion.key_points {
            if let Some(factor) = opinion.opinion_basis.iter()
                .find(|f| f.evidence.contains(point)) {
                formatted.push_str(&format!(
                    "\n- {}: {} (based on {})",
                    point,
                    self.get_factor_explanation(&factor.factor_type),
                    factor.evidence.join(", ")
                ));
            }
        }

        formatted
    }

    fn get_factor_explanation(&self, factor: &FactorType) -> &str {
        match factor {
            FactorType::TechnicalMerit => "from a technical perspective",
            FactorType::UserExperience => "considering user experience",
            FactorType::ValueProposition => "analyzing value for money",
            FactorType::Innovation => "looking at innovation",
            FactorType::Reliability => "considering reliability",
            FactorType::Practicality => "from a practical standpoint",
            FactorType::SecurityImplications => "considering security implications",
        }
    }

    fn format_confidence(&self, score: f32) -> &str {
        match score {
            s if s >= 0.9 => "very high",
            s if s >= 0.7 => "high",
            s if s >= 0.5 => "moderate",
            s if s >= 0.3 => "developing",
            _ => "preliminary",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpinionSystem {
    pub tech_opinions: HashMap<String, FormedOpinion>,
    pub opinion_factors: OpinionFactors,
    pub confidence_levels: HashMap<String, f32>,
    pub experience_data: ExperienceData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormedOpinion {
    pub product_name: String,
    pub opinion_basis: Vec<OpinionFactor>,
    pub confidence_score: f32,
    pub key_points: Vec<String>,
    pub personal_experience: Option<Experience>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub is_sponsored: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpinionFactor {
    pub factor_type: FactorType,
    pub weight: f32,
    pub evidence: Vec<String>,
    pub source_reliability: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FactorType {
    TechnicalMerit,
    UserExperience,
    ValueProposition,
    Innovation,
    Reliability,
    Practicality,
    SecurityImplications,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SponsorshipTracker {
    pub active_sponsorships: HashMap<String, SponsorshipDetails>,
    pub sponsorship_history: Vec<HistoricalSponsorship>,
    pub disclosure_requirements: DisclosurePolicy,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SponsorshipDetails {
    pub sponsor: String,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub terms: Vec<String>,
    pub disclosure_text: String,
}

impl SponsorshipTracker {
    pub async fn check_sponsorship(&self, topic: &str) -> Result<bool, String> {
        // Check if topic is related to any active sponsorships
        for (product, details) in &self.active_sponsorships {
            if topic.to_lowercase().contains(&product.to_lowercase()) {
                return Ok(true);
            }
        }
        Ok(false)
    }
} 