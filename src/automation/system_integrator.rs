use crate::model::automation::software_controller::ModelingSoftwareController;
use crate::games::traits::GameTrainer;
use crate::resource_management::ResourceManager;
use crate::monitoring::performance::PerformanceMonitor;
use crate::error_handling::SystemError;
use crate::automation::{AutomationCoordinator, TaskScheduler};
use tokio::sync::RwLock;
use std::sync::Arc;
use dashmap::DashMap;

pub struct SystemIntegrator {
    coordinator: Arc<AutomationCoordinator>,
    task_scheduler: Arc<TaskScheduler>,
    software_controller: Arc<RwLock<ModelingSoftwareController>>,
    resource_manager: Arc<ResourceManager>,
    performance_monitor: Arc<PerformanceMonitor>,
    active_sessions: DashMap<String, IntegrationSession>,
}

#[derive(Debug)]
struct IntegrationSession {
    session_id: String,
    game_trainer: Arc<dyn GameTrainer>,
    model_state: ModelState,
    resources: ResourceAllocation,
    started_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
struct ModelState {
    current_phase: ModelingPhase,
    completion_percentage: f32,
    active_components: Vec<String>,
    last_update: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
struct ResourceAllocation {
    cpu_limit: f32,
    memory_limit: f64,
    priority_level: Priority,
}

impl SystemIntegrator {
    pub fn new(
        coordinator: Arc<AutomationCoordinator>,
        task_scheduler: Arc<TaskScheduler>,
        resource_manager: Arc<ResourceManager>,
    ) -> Self {
        Self {
            coordinator,
            task_scheduler,
            software_controller: Arc::new(RwLock::new(
                ModelingSoftwareController::new(PathBuf::from("./working_dir")).await
            )),
            resource_manager,
            performance_monitor: Arc::new(PerformanceMonitor::new()),
            active_sessions: DashMap::new(),
        }
    }

    pub async fn start_integration_session(
        &self,
        game_trainer: Arc<dyn GameTrainer>,
        initial_phase: ModelingPhase,
    ) -> Result<String, SystemError> {
        // Generate unique session ID
        let session_id = format!("session_{}", uuid::Uuid::new_v4());

        // Initialize session
        let session = IntegrationSession {
            session_id: session_id.clone(),
            game_trainer,
            model_state: ModelState {
                current_phase: initial_phase,
                completion_percentage: 0.0,
                active_components: Vec::new(),
                last_update: chrono::Utc::now(),
            },
            resources: ResourceAllocation {
                cpu_limit: 0.5,
                memory_limit: 1024.0 * 1024.0 * 512.0, // 512MB
                priority_level: Priority::Medium,
            },
            started_at: chrono::Utc::now(),
        };

        // Store session
        self.active_sessions.insert(session_id.clone(), session);

        // Schedule initial tasks
        self.schedule_integration_tasks(&session_id).await?;

        Ok(session_id)
    }

    async fn schedule_integration_tasks(&self, session_id: &str) -> Result<(), SystemError> {
        let session = self.active_sessions.get(session_id)
            .ok_or_else(|| SystemError::Resource("Session not found".to_string()))?;

        // Schedule modeling task
        self.task_scheduler.schedule_task(AutomationTask {
            task_type: TaskType::ModelCreation(session.model_state.current_phase),
            priority: session.resources.priority_level,
            resource_requirements: ResourceRequirements {
                cpu_usage: session.resources.cpu_limit,
                memory_usage: session.resources.memory_limit,
            },
            status: TaskStatus::Pending,
            started_at: chrono::Utc::now(),
        }).await?;

        // Schedule game training task
        self.task_scheduler.schedule_task(AutomationTask {
            task_type: TaskType::GameTraining(session_id.to_string()),
            priority: session.resources.priority_level,
            resource_requirements: ResourceRequirements {
                cpu_usage: session.resources.cpu_limit,
                memory_usage: session.resources.memory_limit,
            },
            status: TaskStatus::Pending,
            started_at: chrono::Utc::now(),
        }).await?;

        Ok(())
    }

    pub async fn update_session(&self, session_id: &str, phase: ModelingPhase) -> Result<(), SystemError> {
        let mut session = self.active_sessions.get_mut(session_id)
            .ok_or_else(|| SystemError::Resource("Session not found".to_string()))?;

        // Update model state
        session.model_state.current_phase = phase;
        session.model_state.last_update = chrono::Utc::now();

        // Schedule new tasks for updated phase
        self.schedule_integration_tasks(session_id).await?;

        Ok(())
    }

    pub async fn monitor_sessions(&self) -> Result<(), SystemError> {
        for session in self.active_sessions.iter() {
            // Check session health
            let duration = chrono::Utc::now() - session.started_at;
            if duration > chrono::Duration::hours(2) {
                self.handle_long_running_session(&session).await?;
            }

            // Monitor resource usage
            let metrics = self.performance_monitor.get_session_metrics(&session.session_id).await?;
            if metrics.memory_usage > session.resources.memory_limit {
                self.handle_resource_overflow(&session).await?;
            }
        }

        Ok(())
    }

    async fn handle_long_running_session(&self, session: &IntegrationSession) -> Result<(), SystemError> {
        tracing::warn!("Session {} running longer than expected", session.session_id);
        
        // Adjust priority
        self.task_scheduler.adjust_task_priority(
            &session.session_id,
            Priority::High
        ).await?;

        Ok(())
    }

    async fn handle_resource_overflow(&self, session: &IntegrationSession) -> Result<(), SystemError> {
        tracing::warn!("Session {} exceeding resource limits", session.session_id);
        
        // Pause non-critical components
        self.coordinator.pause_session_components(&session.session_id).await?;
        
        // Wait for resources to free up
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        Ok(())
    }
} 