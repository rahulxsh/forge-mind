use std::sync::Arc;

use axum::{body::Body, http::{Request}, Router};
use axum::http::StatusCode;
use tower::ServiceExt;

use api::{
    AppState,
    create_app,
    jobs::channel::JobChannel,
    repositories::documents::DocumentsRepository,
    services::documents::DocumentService,
};

use sqlx::PgPool;
use config::load_config;

async fn test_pool() -> PgPool {
    let config = load_config().unwrap();

    let pool = PgPool::connect(&config.test_db_url)
        .await
        .expect("Failed to connect to test database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

async fn test_app() -> (Router,JobChannel) {
    let pool = test_pool().await;

    let job_channel = JobChannel::new(2);

    let state = AppState {
        tx: job_channel.tx.clone(),
        document_service: Arc::new(DocumentService {
            repository: DocumentsRepository {
                pool,
            },
            tx: job_channel.tx.clone(),
        }),
    };

   let app =  create_app(state);

    (app,job_channel)
}

#[tokio::test]
async fn create_document() {
    let (app,_job_channel) = test_app().await;

    let body = serde_json::json!({
        "file_name": "test.pdf",
        "content_type": "application/pdf"
    });

    let request = Request::post("/api/v1/documents")
        .header("content-type","application/json")
        .body(Body::from(body.to_string()))
        .unwrap();

    let response = app.oneshot(request)
        .await
        .unwrap();

    assert_eq!(response.status(),StatusCode::OK)
}