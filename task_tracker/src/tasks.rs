use std::fmt::Display;

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

#[derive(Debug, Clone, Serialize, Deserialize, Default, clap::ValueEnum, PartialEq)]
pub enum TaskStatus {
    #[default]
    Todo,
    InProgress,
    Done,
    Skip,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "Todo"),
            TaskStatus::InProgress => write!(f, "In Progress"),
            TaskStatus::Done => write!(f, "Done"),
            TaskStatus::Skip => write!(f, "Skip"),
        }
    }
}

pub struct TaskManager {
    tasks: Vec<Task>,
}
