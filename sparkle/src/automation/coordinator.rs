use crate::model::automation::software_controller::ModelingSoftwareController;
use crate::games::traits::GameTrainer;
use crate::resource_management::ResourceManager;
use crate::monitoring::performance::PerformanceMonitor;
use crate::error_handling::SystemError;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AutomationCoordinator {
    resource_manager: Arc<ResourceManager>,
    performance_monitor: Arc<PerformanceMonitor>,
    software_controller: Arc<RwLock<ModelingSoftwareController>>,
    game_trainers: DashMap<String, Arc<dyn GameTrainer>>,
    active_tasks: DashMap<String, AutomationTask>,
}

#[derive(Debug)]
pub struct AutomationTask {
    pub task_type: TaskType,
    pub priority: Priority,
    pub resource_requirements: ResourceRequirements,
    pub status: TaskStatus,
    pub started_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub enum TaskType {
    ModelCreation(ModelingPhase),
    GameTraining(String), // game identifier
    ResourceOptimization,
    SystemMaintenance,
}

#[derive(Debug)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

impl AutomationCoordinator {
    pub async fn new(resource_manager: Arc<ResourceManager>) -> Self {
        let performance_monitor = Arc::new(PerformanceMonitor::new());
        
        Self {
            resource_manager: resource_manager.clone(),
            performance_monitor: performance_monitor.clone(),
            software_controller: Arc::new(RwLock::new(
                ModelingSoftwareController::new(PathBuf::from("./working_dir")).await
            )),
            game_trainers: DashMap::new(),
            active_tasks: DashMap::new(),
        }
    }

    pub async fn coordinate_tasks(&self) -> Result<(), SystemError> {
        // Monitor system resources
        if self.resource_manager.should_throttle() {
            self.pause_non_critical_tasks().await?;
        }

        // Process modeling tasks
        self.process_modeling_tasks().await?;

        // Process game training tasks
        self.process_game_training_tasks().await?;

        // Collect and analyze metrics
        self.collect_system_metrics().await?;

        Ok(())
    }

    async fn process_modeling_tasks(&self) -> Result<(), SystemError> {
        let mut software_controller = self.software_controller.write().await;
        
        for task in self.active_tasks.iter() {
            if let TaskType::ModelCreation(phase) = &task.task_type {
                match phase {
                    ModelingPhase::Face => {
                        software_controller.create_model_part(ModelPart::Face).await
                            .map_err(|e| SystemError::Resource(e))?;
                    },
                    ModelingPhase::Physics => {
                        software_controller.apply_physics(PhysicsComponent::Hair).await
                            .map_err(|e| SystemError::Resource(e))?;
                    },
                    // Handle other phases...
                }
            }
        }
        
        Ok(())
    }

    async fn process_game_training_tasks(&self) -> Result<(), SystemError> {
        for task in self.active_tasks.iter() {
            if let TaskType::GameTraining(game_id) = &task.task_type {
                if let Some(trainer) = self.game_trainers.get(game_id) {
                    if !trainer.should_throttle() {
                        // Process training task
                        let routine = trainer.get_training_routine().await
                            .map_err(|e| SystemError::Resource(e.to_string()))?;
                            
                        // Execute training routine
                        self.execute_training_routine(&routine).await?;
                    }
                }
            }
        }
        
        Ok(())
    }

    async fn pause_non_critical_tasks(&self) -> Result<(), SystemError> {
        for mut task in self.active_tasks.iter_mut() {
            if task.priority != Priority::Critical {
                task.status = TaskStatus::Paused;
            }
        }
        Ok(())
    }

    async fn collect_system_metrics(&self) -> Result<(), SystemError> {
        // Collect metrics from all components
        let modeling_metrics = self.software_controller.read().await.collect_metrics().await?;
        
        for trainer in self.game_trainers.iter() {
            let trainer_metrics = trainer.collect_metrics().await
                .map_err(|e| SystemError::Performance(e.to_string()))?;
            
            self.performance_monitor.record_component_metrics(
                trainer.key(),
                trainer_metrics
            ).await;
        }

        Ok(())
    }
} 