use crate::AppState;
use crate::routes::documents::documents_routes;
use axum::Router;

pub mod documents;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/documents", documents_routes())
}
