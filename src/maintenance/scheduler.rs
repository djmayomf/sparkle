use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use tokio::sync::broadcast;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceTask {
    pub task_type: TaskType,
    pub scheduled_time: DateTime<Utc>,
    pub duration: Duration,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TaskType {
    KnowledgeBaseUpdate,
    ModelUpgrade,
    SecurityAudit,
    BackupCreation,
    PerformanceOptimization,
}

pub struct MaintenanceScheduler {
    tasks: HashMap<TaskType, MaintenanceTask>,
    event_sender: broadcast::Sender<MaintenanceTask>,
    last_maintenance: HashMap<TaskType, DateTime<Utc>>,
}

impl MaintenanceScheduler {
    pub fn new() -> (Self, broadcast::Receiver<MaintenanceTask>) {
        let (tx, rx) = broadcast::channel(100);
        
        (Self {
            tasks: HashMap::new(),
            event_sender: tx,
            last_maintenance: HashMap::new(),
        }, rx)
    }

    pub async fn schedule_task(&mut self, task_type: TaskType, scheduled_time: DateTime<Utc>, duration: Duration) {
        let task = MaintenanceTask {
            task_type: task_type.clone(),
            scheduled_time,
            duration,
            description: self.get_task_description(&task_type),
            completed: false,
        };

        self.tasks.insert(task_type, task.clone());
        self.broadcast_task(task).await;
    }

    pub async fn complete_task(&mut self, task_type: &TaskType) -> Result<(), String> {
        if let Some(task) = self.tasks.get_mut(task_type) {
            task.completed = true;
            self.last_maintenance.insert(task_type.clone(), Utc::now());
            self.broadcast_task(task.clone()).await;
            Ok(())
        } else {
            Err("Task not found".to_string())
        }
    }

    fn get_task_description(&self, task_type: &TaskType) -> String {
        match task_type {
            TaskType::KnowledgeBaseUpdate => "Updating knowledge base with new content".to_string(),
            TaskType::ModelUpgrade => "Upgrading VTuber model and animations".to_string(),
            TaskType::SecurityAudit => "Performing security audit and updates".to_string(),
            TaskType::BackupCreation => "Creating system backup".to_string(),
            TaskType::PerformanceOptimization => "Optimizing system performance".to_string(),
        }
    }

    pub fn get_pending_tasks(&self) -> Vec<&MaintenanceTask> {
        self.tasks.values()
            .filter(|task| !task.completed)
            .collect()
    }

    pub fn get_last_maintenance(&self, task_type: &TaskType) -> Option<&DateTime<Utc>> {
        self.last_maintenance.get(task_type)
    }

    async fn broadcast_task(&self, task: MaintenanceTask) {
        let _ = self.event_sender.send(task);
    }

    pub fn should_schedule_maintenance(&self, task_type: &TaskType) -> bool {
        if let Some(last_time) = self.last_maintenance.get(task_type) {
            match task_type {
                TaskType::KnowledgeBaseUpdate => Utc::now() - *last_time > Duration::days(7),
                TaskType::SecurityAudit => Utc::now() - *last_time > Duration::days(14),
                TaskType::BackupCreation => Utc::now() - *last_time > Duration::days(1),
                TaskType::PerformanceOptimization => Utc::now() - *last_time > Duration::days(30),
                TaskType::ModelUpgrade => Utc::now() - *last_time > Duration::days(90),
            }
        } else {
            true
        }
    }
} 