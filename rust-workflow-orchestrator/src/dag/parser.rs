// This file implements functionality to parse DAG definitions from YAML or a Rust macro-based DSL.

use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dag {
    pub name: String,
    pub schedule: String,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub command: String,
    pub dependencies: Vec<String>,
}

impl Dag {
    pub fn from_yaml(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(file_path)?;
        let dag: Dag = serde_yaml::from_str(&contents)?;
        Ok(dag)
    }
}