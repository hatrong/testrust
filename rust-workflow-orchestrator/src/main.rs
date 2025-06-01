// src/main.rs
use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::task;
use crate::scheduler::CronScheduler;
use crate::web::routes;

#[tokio::main]
async fn main() {
    // Initialize the logging
    crate::utils::logging::init_logging();

    // Set up the scheduler
    let scheduler = CronScheduler::new();
    task::spawn(async move {
        scheduler.start().await;
    });

    // Set up the web server
    let app = Router::new()
        .merge(routes::create_routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}