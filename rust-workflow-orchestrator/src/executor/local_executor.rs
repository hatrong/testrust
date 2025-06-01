// src/executor/local_executor.rs

use tokio::process::Command;
use std::process::ExitStatus;
use std::error::Error;

pub struct LocalExecutor;

impl LocalExecutor {
    pub async fn execute_command(command: &str, args: &[&str]) -> Result<ExitStatus, Box<dyn Error>> {
        let mut cmd = Command::new(command);
        cmd.args(args);
        
        let status = cmd.status().await?;
        Ok(status)
    }

    pub async fn execute_rust_function<F, R>(&self, func: F) -> Result<R, Box<dyn Error>>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let result = tokio::task::spawn_blocking(func).await?;
        Ok(result)
    }
}