use crate::automation::coordinator::AutomationCoordinator;
use crate::error_handling::SystemError;
use crate::resource_management::ResourceManager;
use crate::monitoring::performance::PerformanceMonitor;
use tokio::time::{interval, Duration};
use std::sync::Arc;
use dashmap::DashMap;

pub struct TaskScheduler {
    coordinator: Arc<AutomationCoordinator>,
    resource_manager: Arc<ResourceManager>,
    performance_monitor: Arc<PerformanceMonitor>,
    task_queue: DashMap<Priority, Vec<AutomationTask>>,
    running_tasks: DashMap<String, TaskHandle>,
}

#[derive(Debug)]
struct TaskHandle {
    task_id: String,
    priority: Priority,
    handle: tokio::task::JoinHandle<Result<(), SystemError>>,
    started_at: chrono::DateTime<chrono::Utc>,
}

impl TaskScheduler {
    pub fn new(
        coordinator: Arc<AutomationCoordinator>,
        resource_manager: Arc<ResourceManager>,
        performance_monitor: Arc<PerformanceMonitor>,
    ) -> Self {
        Self {
            coordinator,
            resource_manager,
            performance_monitor,
            task_queue: DashMap::new(),
            running_tasks: DashMap::new(),
        }
    }

    pub async fn start(&self) -> Result<(), SystemError> {
        let mut interval = interval(Duration::from_millis(100));

        loop {
            interval.tick().await;

            // Check system resources
            self.check_resource_usage().await?;

            // Process queued tasks
            self.process_task_queue().await?;

            // Monitor running tasks
            self.monitor_running_tasks().await?;

            // Clean up completed tasks
            self.cleanup_tasks().await?;
        }
    }

    async fn check_resource_usage(&self) -> Result<(), SystemError> {
        if self.resource_manager.should_throttle() {
            // Pause low priority tasks
            self.pause_low_priority_tasks().await?;

            // Wait for resources to free up
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        Ok(())
    }

    async fn process_task_queue(&self) -> Result<(), SystemError> {
        // Process tasks in priority order
        for priority in [Priority::Critical, Priority::High, Priority::Medium, Priority::Low] {
            if let Some(mut tasks) = self.task_queue.get_mut(&priority) {
                while let Some(task) = tasks.pop() {
                    if self.can_start_task(&task).await? {
                        self.start_task(task).await?;
                    } else {
                        // Put task back in queue if can't start
                        tasks.push(task);
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    async fn can_start_task(&self, task: &AutomationTask) -> Result<bool, SystemError> {
        // Check system resources
        if self.resource_manager.should_throttle() && task.priority != Priority::Critical {
            return Ok(false);
        }

        // Check task-specific requirements
        match &task.task_type {
            TaskType::ModelCreation(_) => {
                self.check_modeling_resources(task).await
            },
            TaskType::GameTraining(game_id) => {
                self.check_game_training_resources(game_id, task).await
            },
            TaskType::ResourceOptimization => Ok(true),
            TaskType::SystemMaintenance => {
                self.check_maintenance_resources(task).await
            },
        }
    }

    async fn start_task(&self, task: AutomationTask) -> Result<(), SystemError> {
        let task_id = uuid::Uuid::new_v4().to_string();
        let coordinator = self.coordinator.clone();
        
        let handle = tokio::spawn(async move {
            match task.task_type {
                TaskType::ModelCreation(phase) => {
                    coordinator.process_modeling_task(phase).await
                },
                TaskType::GameTraining(game_id) => {
                    coordinator.process_game_training(&game_id).await
                },
                TaskType::ResourceOptimization => {
                    coordinator.optimize_resources().await
                },
                TaskType::SystemMaintenance => {
                    coordinator.perform_maintenance().await
                },
            }
        });

        self.running_tasks.insert(task_id.clone(), TaskHandle {
            task_id,
            priority: task.priority,
            handle,
            started_at: chrono::Utc::now(),
        });

        Ok(())
    }

    async fn monitor_running_tasks(&self) -> Result<(), SystemError> {
        for task in self.running_tasks.iter() {
            // Check if task has been running too long
            let duration = chrono::Utc::now() - task.started_at;
            if duration > chrono::Duration::hours(1) {
                self.handle_stuck_task(task).await?;
            }

            // Monitor task resource usage
            self.monitor_task_resources(task).await?;
        }
        Ok(())
    }

    async fn cleanup_tasks(&self) -> Result<(), SystemError> {
        let mut completed_tasks = Vec::new();

        for task in self.running_tasks.iter() {
            if task.handle.is_finished() {
                // Get task result
                match task.handle.await {
                    Ok(result) => {
                        if let Err(e) = result {
                            tracing::error!("Task {} failed: {:?}", task.task_id, e);
                        }
                    },
                    Err(e) => {
                        tracing::error!("Task join error: {:?}", e);
                    }
                }
                completed_tasks.push(task.task_id.clone());
            }
        }

        // Remove completed tasks
        for task_id in completed_tasks {
            self.running_tasks.remove(&task_id);
        }

        Ok(())
    }

    async fn handle_stuck_task(&self, task: &TaskHandle) -> Result<(), SystemError> {
        tracing::warn!("Task {} appears stuck, attempting recovery", task.task_id);

        // Try graceful shutdown first
        if let Err(e) = self.coordinator.gracefully_stop_task(&task.task_id).await {
            tracing::error!("Failed to gracefully stop task: {:?}", e);
            
            // Force stop if graceful shutdown fails
            task.handle.abort();
        }

        Ok(())
    }

    async fn monitor_task_resources(&self, task: &TaskHandle) -> Result<(), SystemError> {
        let metrics = self.performance_monitor.get_task_metrics(&task.task_id).await?;

        if metrics.memory_usage > 0.9 || metrics.cpu_usage > 0.9 {
            tracing::warn!("Task {} using excessive resources", task.task_id);
            self.handle_resource_intensive_task(task).await?;
        }

        Ok(())
    }
} 