use crate::AppState;
use crate::handlers::documents::add_document;
use axum::Router;
use axum::routing::post;

pub fn documents_routes() -> Router<AppState> {
    Router::new().route("/", post(add_document))
}
