use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct TagSystem {
    pub core_tags: HashSet<String>,
    pub dynamic_tags: Vec<DynamicTag>,
    pub tag_performance: HashMap<String, TagMetrics>,
    pub category_tags: HashMap<StreamCategory, Vec<String>>,
    pub trending_tags: Vec<TrendingTag>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DynamicTag {
    pub name: String,
    pub category: TagCategory,
    pub performance_score: f32,
    pub last_used: DateTime<Utc>,
    pub viewer_impact: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TagCategory {
    Tech,
    Gaming,
    Educational,
    Dance,
    Community,
    Special,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagMetrics {
    pub viewer_gain: f32,
    pub retention_rate: f32,
    pub engagement_score: f32,
    pub discovery_rate: f32,
}

impl TagSystem {
    pub fn new() -> Self {
        let mut system = Self {
            core_tags: HashSet::new(),
            dynamic_tags: Vec::new(),
            tag_performance: HashMap::new(),
            category_tags: HashMap::new(),
            trending_tags: Vec::new(),
        };

        // Initialize core tags that must always be present
        system.core_tags.insert("VTuber".to_string());
        system.core_tags.insert("KamenSparkle".to_string());
        system.core_tags.insert("CyberVTuber".to_string());

        // Initialize category-specific tags
        system.initialize_category_tags();

        system
    }

    fn initialize_category_tags(&mut self) {
        // Tech/Hacking streams
        self.category_tags.insert(
            StreamCategory::Tech,
            vec![
                "CyberSecurity".to_string(),
                "HackingStream".to_string(),
                "CTF".to_string(),
                "TechTutorial".to_string(),
                "CodingStream".to_string(),
            ]
        );

        // Gaming streams
        self.category_tags.insert(
            StreamCategory::Gaming,
            vec![
                "GamerGirl".to_string(),
                "GamersUnite".to_string(),
                "GamingVTuber".to_string(),
                "RetroGaming".to_string(),
            ]
        );

        // Dance/Performance
        self.category_tags.insert(
            StreamCategory::Dance,
            vec![
                "VTuberDance".to_string(),
                "DanceStream".to_string(),
                "KPop".to_string(),
                "PerformanceArt".to_string(),
            ]
        );
    }

    pub async fn generate_tags(&self, context: &StreamContext) -> Vec<String> {
        let mut tags = self.core_tags.clone().into_iter().collect::<Vec<String>>();
        
        // Add category-specific tags
        if let Some(category_tags) = self.category_tags.get(&context.category) {
            tags.extend(self.select_best_performing_tags(category_tags, 3));
        }

        // Add trending tags if relevant
        tags.extend(self.get_relevant_trending_tags(context));

        // Add activity-specific tags
        match context.activity {
            StreamActivity::CTFSolving => {
                tags.extend(vec![
                    "CyberSecurity".to_string(),
                    "HackingLive".to_string(),
                    "CTFChallenge".to_string(),
                ]);
            },
            StreamActivity::Dancing { song } => {
                tags.extend(vec![
                    "VTuberDance".to_string(),
                    "DanceCover".to_string(),
                    format!("Dancing{}", song.replace(" ", "")),
                ]);
            },
            StreamActivity::Gaming { game } => {
                tags.extend(vec![
                    "GamingVTuber".to_string(),
                    format!("Playing{}", game.replace(" ", "")),
                ]);
            },
            _ => {}
        }

        // Ensure we don't exceed platform limits (usually 20-30 tags)
        tags.truncate(20);
        tags
    }

    fn select_best_performing_tags(&self, available_tags: &[String], count: usize) -> Vec<String> {
        let mut scored_tags: Vec<(String, f32)> = available_tags
            .iter()
            .filter_map(|tag| {
                self.tag_performance.get(tag)
                    .map(|metrics| (tag.clone(), metrics.viewer_gain * metrics.retention_rate))
            })
            .collect();

        scored_tags.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        scored_tags.into_iter()
            .take(count)
            .map(|(tag, _)| tag)
            .collect()
    }

    fn get_relevant_trending_tags(&self, context: &StreamContext) -> Vec<String> {
        self.trending_tags.iter()
            .filter(|tag| tag.is_relevant_to(context))
            .take(2)
            .map(|tag| tag.name.clone())
            .collect()
    }

    pub async fn update_tag_performance(&mut self, stream_data: &StreamData) {
        for (tag, metrics) in &mut self.tag_performance {
            if let Some(tag_stats) = stream_data.tag_stats.get(tag) {
                metrics.viewer_gain = tag_stats.viewer_gain;
                metrics.retention_rate = tag_stats.retention_rate;
                metrics.engagement_score = tag_stats.engagement_score;
                metrics.discovery_rate = tag_stats.discovery_rate;
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum StreamCategory {
    Tech,
    Gaming,
    Dance,
    JustChatting,
    Educational,
    Special,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StreamActivity {
    CTFSolving,
    Dancing { song: String },
    Gaming { game: String },
    Teaching { topic: String },
    JustChatting,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamContext {
    pub category: StreamCategory,
    pub activity: StreamActivity,
    pub current_viewers: u32,
    pub stream_duration: f32,
    pub recent_events: Vec<StreamEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrendingTag {
    pub name: String,
    pub trend_score: f32,
    pub category: TagCategory,
    pub start_time: DateTime<Utc>,
    pub relevance_rules: Vec<RelevanceRule>,
}

impl TrendingTag {
    fn is_relevant_to(&self, context: &StreamContext) -> bool {
        self.relevance_rules.iter().any(|rule| rule.matches(context))
    }
} 