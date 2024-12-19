use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ViralContentGenerator {
    pub trends: TrendAnalyzer,
    pub meme_factory: MemeGenerator,
    pub clip_detector: ClipDetector,
    pub viral_hooks: Vec<ViralHook>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrendAnalyzer {
    pub current_trends: Vec<Trend>,
    pub trend_predictions: Vec<TrendPrediction>,
    pub viral_potential: HashMap<String, f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemeGenerator {
    pub templates: Vec<MemeTemplate>,
    pub timing_strategies: Vec<TimingStrategy>,
    pub crossover_opportunities: Vec<Crossover>,
}

impl ViralContentGenerator {
    pub fn new() -> Self {
        Self {
            trends: TrendAnalyzer::new(),
            meme_factory: MemeGenerator::new(),
            clip_detector: ClipDetector::new(),
            viral_hooks: vec![
                ViralHook {
                    trigger: "Unexpected hack success".to_string(),
                    response_type: ResponseType::TechExplanation,
                    timing: TimingStrategy::Immediate,
                    follow_up: vec![
                        "Teach chat basic concept".to_string(),
                        "Create relatable meme".to_string(),
                    ],
                },
                ViralHook {
                    trigger: "Chat teaches something new".to_string(),
                    response_type: ResponseType::EmotionalReaction,
                    timing: TimingStrategy::Natural,
                    follow_up: vec![
                        "Thank chat genuinely".to_string(),
                        "Show immediate application".to_string(),
                    ],
                },
            ],
        }
    }

    pub async fn generate_viral_moment(&mut self, context: &StreamContext) -> Option<ViralContent> {
        if let Some(hook) = self.detect_viral_opportunity(context) {
            Some(self.create_viral_content(&hook))
        } else {
            None
        }
    }

    fn detect_viral_opportunity(&self, context: &StreamContext) -> Option<&ViralHook> {
        // Analyze stream context for viral potential
        self.viral_hooks.iter().find(|hook| {
            self.matches_trigger(hook, context)
        })
    }

    fn create_viral_content(&self, hook: &ViralHook) -> ViralContent {
        ViralContent {
            type_: ContentType::TechMeme,
            hook: hook.clone(),
            timing: Utc::now(),
            follow_up_strategy: self.generate_follow_up(hook),
        }
    }
} 