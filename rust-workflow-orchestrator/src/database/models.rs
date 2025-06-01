// src/database/models.rs

use serde::{Deserialize, Serialize};

// Represents a Directed Acyclic Graph (DAG) in the database
#[derive(Debug, Serialize, Deserialize)]
pub struct Dag {
    pub id: i32,
    pub name: String,
    pub schedule: String, // Cron expression
}

// Represents a task within a DAG
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub dag_id: i32, // Foreign key to the DAG
    pub task_id: String,
    pub command: String,
}

// Represents an instance of a task execution
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskInstance {
    pub id: i32,
    pub task_id: i32, // Foreign key to the Task
    pub execution_time: chrono::NaiveDateTime,
    pub state: TaskState,
}

// Enum representing the possible states of a task instance
#[derive(Debug, Serialize, Deserialize)]
pub enum TaskState {
    Pending,
    Running,
    Completed,
    Failed,
}