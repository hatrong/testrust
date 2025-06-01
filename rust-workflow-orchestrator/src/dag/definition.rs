// This file defines the structure for Directed Acyclic Graphs (DAGs) used in the workflow orchestration system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub command: String,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dag {
    pub name: String,
    pub schedule: String, // Cron expression
    pub tasks: HashMap<String, Task>, // Task ID to Task mapping
}

impl Dag {
    pub fn new(name: String, schedule: String) -> Self {
        Dag {
            name,
            schedule,
            tasks: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.insert(task.id.clone(), task);
    }
}