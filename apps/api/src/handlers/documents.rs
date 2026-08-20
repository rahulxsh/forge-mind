use crate::AppState;
use crate::error::AppError;
use crate::models::documents::{DocumentRequest, DocumentResponse};
use axum::Json;
use axum::extract::State;

#[axum::debug_handler]
pub async fn add_document(
    State(state): State<AppState>,
    Json(payload): Json<DocumentRequest>,
) -> Result<Json<DocumentResponse>, AppError> {
    let document = state.document_service.add_document(payload).await?;

    Ok(Json(document))
}
