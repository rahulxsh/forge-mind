use crate::AppState;
use crate::error::AppError;
use crate::models::documents::{DocumentRequest, DocumentResponse};
use axum::Json;
use axum::extract::Path;
use axum::extract::State;
use uuid::Uuid;

#[axum::debug_handler]
pub async fn add_document(
    State(state): State<AppState>,
    Json(payload): Json<DocumentRequest>,
) -> Result<Json<DocumentResponse>, AppError> {
    let document = state.document_service.add_document(payload).await?;
    Ok(Json(document))
}

pub async fn get_documents(
    State(state): State<AppState>,
) -> Result<Json<Vec<DocumentResponse>>, AppError> {
    let docs = state.document_service.get_documents().await?;

    Ok(Json(docs))
}

pub async fn get_document_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<DocumentResponse>, AppError> {
    let doc = state.document_service.get_document_by_id(id).await?;

    Ok(Json(doc))
}
