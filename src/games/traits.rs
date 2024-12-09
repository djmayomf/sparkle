use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::resource_management::ResourceManager;
use crate::error::Result;

#[async_trait]
pub trait GameTrainer: Send + Sync {
    // Core functionality
    async fn get_advice(&self, context: &str) -> Result<String>;
    async fn analyze_gameplay(&self, gameplay_data: &GameplayData) -> Result<Analysis>;
    async fn get_training_routine(&self) -> Result<TrainingRoutine>;
    
    // Resource management
    fn get_resource_usage(&self) -> ResourceUsage;
    fn should_throttle(&self) -> bool;
    
    // Performance monitoring
    async fn collect_metrics(&self) -> Result<PerformanceMetrics>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameplayData {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub game_type: String,
    pub player_actions: Vec<PlayerAction>,
    pub match_outcome: Option<MatchOutcome>,
    pub performance_metrics: PerformanceData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Analysis {
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub improvement_areas: Vec<String>,
    pub recommended_focus: String,
    pub confidence_score: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingRoutine {
    pub exercises: Vec<Exercise>,
    pub duration: std::time::Duration,
    pub difficulty: u8,
    pub focus_areas: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_usage: f32,
    pub memory_usage: f64,
    pub cache_size: usize,
    pub network_usage: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub response_time: std::time::Duration,
    pub cache_hit_rate: f32,
    pub error_rate: f32,
    pub throughput: u32,
} 