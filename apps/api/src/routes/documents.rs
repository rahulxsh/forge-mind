use crate::AppState;
use crate::handlers::documents::{add_document, get_document_by_id, get_documents};
use axum::Router;
use axum::routing::{get, post};

pub fn documents_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(add_document))
        .route("/", get(get_documents))
        .route("/{id}", get(get_document_by_id))
}
