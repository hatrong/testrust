// This file sets up the routes for the REST API endpoints.

use axum::{routing::Router, routing::get, routing::post};
use crate::web::handlers::{list_dags, view_task_status, trigger_manual_run};

pub fn create_routes() -> Router {
    Router::new()
        .route("/api/dags", get(list_dags))
        .route("/api/task_instances/:id", get(view_task_status))
        .route("/api/dags/:dag_id/run", post(trigger_manual_run))
}