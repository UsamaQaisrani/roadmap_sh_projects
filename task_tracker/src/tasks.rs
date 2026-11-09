use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Task {
    pub id: u64,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum TaskStatus {
    #[default]
    Todo,
    InProgress,
    Done,
    Skip,
}

pub struct TaskManager {
    tasks: Vec<Task>,
}
