use crate::jobs::channel::Job;
use crate::jobs::job::start_job;
use crate::routes::routes;
use crate::services::documents::DocumentService;
use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;
use tokio::sync::mpsc;

pub mod constants;
pub mod db;
pub mod error;
pub mod handlers;
pub mod jobs;
pub mod models;
pub mod repositories;
pub mod response;
pub mod routes;
pub mod services;
pub mod workers;

#[derive(Clone)]
pub struct AppState {
    pub tx: mpsc::Sender<Job>,
    pub document_service: Arc<DocumentService>,
}

pub fn create_app(state: AppState) -> Router {
    let app: Router = Router::new()
        .nest("/api/v1", routes())
        .route("/health", get(health))
        .route("/job", post(start_job))
        .with_state(state);

    app
}

pub async fn health() -> &'static str {
    "Health"
}
