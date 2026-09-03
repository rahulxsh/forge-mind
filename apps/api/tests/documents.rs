use std::sync::Arc;

use axum::http::StatusCode;
use axum::{Router, body, body::Body, http::Request};
use tower::ServiceExt;

use api::{
    AppState, create_app, jobs::channel::JobChannel, repositories::documents::DocumentsRepository,
    services::documents::DocumentService,
};

use config::load_config;
use extractor::DataLabExtractor;
use sqlx::PgPool;
use uuid::Uuid;

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

fn init_extractor() -> DataLabExtractor {
    let config = load_config().unwrap();
    let extractor = DataLabExtractor::new(config.datalab_api_key);
    extractor
}

async fn test_app() -> (Router, JobChannel) {
    let pool = test_pool().await;

    let job_channel = JobChannel::new(2);
    let extractor = init_extractor();

    let state = AppState {
        tx: job_channel.tx.clone(),
        document_service: Arc::new(DocumentService {
            repository: DocumentsRepository { pool },
            tx: job_channel.tx.clone(),
        }),
        extractor: Arc::new(extractor),
    };

    let app = create_app(state);

    (app, job_channel)
}

#[tokio::test]
async fn create_document() {
    let (app, _job_channel) = test_app().await;

    let file = tokio::fs::read("tests/fixtures/test.md").await.unwrap();

    let boundary = "----forge-mind-test-boundary";

    let mut body = Vec::new();

    body.extend_from_slice(
        format!(
            "--{boundary}\r\n\
         Content-Disposition: form-data; name=\"file\"; filename=\"test.md\"\r\n\
         Content-Type: text/markdown\r\n\
         \r\n"
        )
        .as_bytes(),
    );

    body.extend_from_slice(&file);

    body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());

    let request = Request::post("/api/v1/documents")
        .header(
            "content-type",
            format!("multipart/form-data; boundary={boundary}"),
        )
        .body(Body::from(body))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    println!("status: {}", response.status());

    assert_eq!(StatusCode::OK, StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    println!("body: {}", String::from_utf8_lossy(&body));

    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["message"], "Document created successfully");
    assert_eq!(body["success"], true);
}

#[tokio::test]
async fn create_invalid_document() {
    let (app, _job_channel) = test_app().await;

    let file = tokio::fs::read("tests/fixtures/test.txt").await.unwrap();

    let boundary = "----forge-mind-test-boundary";

    let mut body = Vec::new();

    body.extend_from_slice(
        format!(
            "--{boundary}\r\n\
         Content-Disposition: form-data; name=\"file\"; filename=\"test.txt\"\r\n\
         Content-Type: text/markdown\r\n\
         \r\n"
        )
        .as_bytes(),
    );

    body.extend_from_slice(&file);

    body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());

    let request = Request::post("/api/v1/documents")
        .header(
            "content-type",
            format!("multipart/form-data; boundary={boundary}"),
        )
        .body(Body::from(body))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["success"], false);
}

#[tokio::test]
async fn get_documents() {
    let (app, _job_channel) = test_app().await;

    let request = Request::get("/api/v1/documents")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["message"], "Documents fetched successfully");
    assert!(body["data"].is_array());
}

#[tokio::test]
async fn get_document_by_id() {
    let (app, _job_channel) = test_app().await;

    let file = tokio::fs::read("tests/fixtures/test.md").await.unwrap();

    let boundary = "----forge-mind-test-boundary";

    let mut body = Vec::new();

    body.extend_from_slice(
        format!(
            "--{boundary}\r\n\
         Content-Disposition: form-data; name=\"file\"; filename=\"test.md\"\r\n\
         Content-Type: text/markdown\r\n\
         \r\n"
        )
        .as_bytes(),
    );

    body.extend_from_slice(&file);

    body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());

    let request = Request::post("/api/v1/documents")
        .header(
            "content-type",
            format!("multipart/form-data; boundary={boundary}"),
        )
        .body(Body::from(body))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

    let id = body["data"]["id"].as_str().unwrap();

    let request = Request::get(format!("/api/v1/documents/{id}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["message"], "Document fetch success");
    assert_eq!(body["data"]["id"], id);
}

#[tokio::test]
async fn get_missing_document() {
    let (app, _job_channel) = test_app().await;

    let id = Uuid::new_v4();

    let request = Request::get(format!("/api/v1/documents/{id}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["message"], "Document not found");
}
