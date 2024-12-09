use crate::games::traits::{GameTrainer, GameplayData, Analysis, TrainingRoutine};
use crate::resource_management::ResourceManager;
use async_trait::async_trait;
use dashmap::DashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct POE2Trainer {
    resource_manager: Arc<ResourceManager>,
    builds: DashMap<String, Build>,
    cache: Arc<DashMap<String, CachedResponse>>,
    performance_monitor: Arc<PerformanceMonitor>,
    crafting_knowledge: Arc<CraftingSystem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Build {
    pub class: String,
    pub main_skills: Vec<String>,
    pub passive_tree: Vec<String>,
    pub gear_requirements: Vec<GearPiece>,
    pub leveling_path: Vec<String>,
    pub difficulty: u8,
    pub performance_metrics: BuildMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildMetrics {
    pub clear_speed: f32,
    pub boss_dps: u32,
    pub survivability: f32,
    pub cost_efficiency: f32,
}

impl POE2Trainer {
    pub fn new(resource_manager: Arc<ResourceManager>) -> Self {
        Self {
            resource_manager,
            builds: Self::init_builds(),
            cache: Arc::new(DashMap::new()),
            performance_monitor: Arc::new(PerformanceMonitor::new()),
            crafting_knowledge: Arc::new(CraftingSystem::new()),
        }
    }

    async fn generate_build_advice(&self, build_name: &str) -> Result<String> {
        // Check cache first
        if let Some(cached) = self.check_cache(build_name) {
            self.performance_monitor.record_cache_hit();
            return Ok(cached);
        }

        // Check resource usage
        if self.resource_manager.should_throttle() {
            return Ok("System is busy, please try again in a moment! 💫".to_string());
        }

        // Generate advice
        let build = self.builds.get(build_name)
            .ok_or_else(|| Error::NotFound(format!("Build {} not found", build_name)))?;

        let advice = self.create_build_advice(&build);
        
        // Cache the result
        self.cache_response(build_name, &advice);
        
        Ok(advice)
    }

    fn create_build_advice(&self, build: &Build) -> String {
        format!(
            "yo bestie, here's the scoop on the {} build! 🎮\n\n\
            Main Skills: {}\n\
            Difficulty: {}/10\n\n\
            Leveling Tips:\n{}\n\n\
            Gear Priority:\n{}\n\n\
            Performance Metrics:\n\
            - Clear Speed: {:.1}/10\n\
            - Boss DPS: {}k\n\
            - Survivability: {:.1}/10\n\
            - Cost Efficiency: {:.1}/10\n\n\
            fr fr this build is kinda cracked! 💫",
            build.class,
            build.main_skills.join(", "),
            build.difficulty,
            build.leveling_path.join("\n- "),
            build.gear_requirements.iter()
                .map(|g| format!("- {}", g.requirements.join(", ")))
                .collect::<Vec<_>>()
                .join("\n"),
            build.performance_metrics.clear_speed,
            build.performance_metrics.boss_dps / 1000,
            build.performance_metrics.survivability,
            build.performance_metrics.cost_efficiency
        )
    }
}

#[async_trait]
impl GameTrainer for POE2Trainer {
    async fn get_advice(&self, context: &str) -> Result<String> {
        let start = std::time::Instant::now();
        
        // Extract build name from context
        let build_name = self.extract_build_name(context)?;
        
        // Generate advice
        let advice = self.generate_build_advice(&build_name).await?;
        
        // Record metrics
        self.performance_monitor.record_response_time(start.elapsed());
        
        Ok(advice)
    }

    async fn analyze_gameplay(&self, gameplay_data: &GameplayData) -> Result<Analysis> {
        // Check resource usage
        if self.resource_manager.should_throttle() {
            return Err(Error::ResourceBusy("System is under heavy load".to_string()));
        }

        let analysis = Analysis {
            strengths: self.analyze_strengths(gameplay_data),
            weaknesses: self.analyze_weaknesses(gameplay_data),
            improvement_areas: self.identify_improvement_areas(gameplay_data),
            recommended_focus: self.determine_focus(gameplay_data),
            confidence_score: self.calculate_confidence_score(gameplay_data),
        };

        Ok(analysis)
    }

    async fn get_training_routine(&self) -> Result<TrainingRoutine> {
        Ok(TrainingRoutine {
            exercises: vec![
                Exercise {
                    name: "Boss Mechanics".to_string(),
                    duration: std::time::Duration::from_mins(20),
                    description: "Practice boss fight mechanics".to_string(),
                },
                Exercise {
                    name: "Clear Speed".to_string(),
                    duration: std::time::Duration::from_mins(15),
                    description: "Optimize map clearing efficiency".to_string(),
                },
                Exercise {
                    name: "Crafting Practice".to_string(),
                    duration: std::time::Duration::from_mins(25),
                    description: "Practice optimal crafting strategies".to_string(),
                },
            ],
            duration: std::time::Duration::from_mins(60),
            difficulty: 4,
            focus_areas: vec![
                "Boss Mechanics".to_string(),
                "Clear Speed".to_string(),
                "Crafting".to_string(),
            ],
        })
    }

    fn get_resource_usage(&self) -> ResourceUsage {
        ResourceUsage {
            cpu_usage: self.resource_manager.get_cpu_usage(),
            memory_usage: self.resource_manager.get_memory_usage(),
            cache_size: self.cache.len(),
            network_usage: self.get_network_usage(),
        }
    }

    fn should_throttle(&self) -> bool {
        self.resource_manager.should_throttle()
    }

    async fn collect_metrics(&self) -> Result<PerformanceMetrics> {
        Ok(self.performance_monitor.get_metrics().await)
    }
}

// Helper implementations
impl POE2Trainer {
    fn extract_build_name(&self, context: &str) -> Result<String> {
        // Simple extraction - could be enhanced with NLP
        let context = context.to_lowercase();
        if let Some(build) = context.split_whitespace()
            .skip_while(|&word| word != "build")
            .nth(1)
        {
            Ok(build.to_string())
        } else {
            Err(Error::InvalidInput("Could not find build name in context".to_string()))
        }
    }

    fn analyze_strengths(&self, gameplay: &GameplayData) -> Vec<String> {
        let mut strengths = Vec::new();
        
        // Analyze clear speed
        if let Some(clear_speed) = gameplay.performance_metrics.get_clear_speed() {
            if clear_speed > 8.0 {
                strengths.push("Excellent clear speed".to_string());
            }
        }

        // Analyze boss damage
        if let Some(boss_dps) = gameplay.performance_metrics.get_boss_dps() {
            if boss_dps > 1_000_000 {
                strengths.push("Strong boss damage".to_string());
            }
        }

        strengths
    }

    fn get_network_usage(&self) -> f32 {
        // Implementation for network usage monitoring
        0.0 // Placeholder
    }
} 