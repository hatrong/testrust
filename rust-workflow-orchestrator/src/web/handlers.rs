// This file implements the REST API handlers for listing DAGs, viewing task statuses, and triggering manual runs.

use axum::{
    extract::{Json, Path},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde_json::json;
use crate::database::{self, models::{DAG, TaskInstance}};
use crate::scheduler::Scheduler;

pub async fn list_dags() -> impl IntoResponse {
    match database::get_all_dags().await {
        Ok(dags) => Json(dags),
        Err(err) => {
            log::error!("Failed to fetch DAGs: {}", err);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch DAGs")
        }
    }
}

pub async fn view_task_statuses(Path(dag_id): Path<String>) -> impl IntoResponse {
    match database::get_task_instances_by_dag(&dag_id).await {
        Ok(task_instances) => Json(task_instances),
        Err(err) => {
            log::error!("Failed to fetch task statuses for DAG {}: {}", dag_id, err);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch task statuses")
        }
    }
}

pub async fn trigger_manual_run(Path(dag_id): Path<String>) -> impl IntoResponse {
    match Scheduler::trigger_dag(&dag_id).await {
        Ok(_) => (axum::http::StatusCode::OK, "Triggered manual run"),
        Err(err) => {
            log::error!("Failed to trigger manual run for DAG {}: {}", dag_id, err);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to trigger manual run")
        }
    }
}

pub fn create_routes() -> Router {
    Router::new()
        .route("/api/dags", get(list_dags))
        .route("/api/task_instances/:dag_id", get(view_task_statuses))
        .route("/api/dags/:dag_id/run", post(trigger_manual_run))
}