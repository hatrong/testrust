// This file implements logging functionality using the log crate and env_logger.

use log::{info, error, warn, debug};
use std::env;

pub fn init_logging() {
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    env_logger::Builder::new()
        .filter_level(log_level.parse().unwrap_or(log::LevelFilter::Info))
        .init();
}

pub fn log_task_start(task_id: &str) {
    info!("Starting task: {}", task_id);
}

pub fn log_task_complete(task_id: &str) {
    info!("Completed task: {}", task_id);
}

pub fn log_task_error(task_id: &str, error: &str) {
    error!("Error in task {}: {}", task_id, error);
}

pub fn log_task_warning(task_id: &str, warning: &str) {
    warn!("Warning in task {}: {}", task_id, warning);
}

pub fn log_debug_info(message: &str) {
    debug!("{}", message);
}