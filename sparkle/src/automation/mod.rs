pub mod task_manager;

// Re-export commonly used types
pub use task_manager::{
    TaskManager,
    AutomatedTask,
    TaskType,
    TaskPriority,
    TaskSchedule,
    TaskStatus,
}; 