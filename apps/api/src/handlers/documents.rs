use crate::AppState;
use crate::error::AppError;
use crate::models::documents::{DocumentRequest, DocumentResponse};
use crate::response::ApiResponse;
use axum::Json;
use axum::extract::Path;
use axum::extract::State;
use uuid::Uuid;
use validator::Validate;

#[axum::debug_handler]
pub async fn add_document(
    State(state): State<AppState>,
    Json(payload): Json<DocumentRequest>,
) -> Result<Json<ApiResponse<DocumentResponse>>, AppError> {
    payload.validate()?;
    let document = state.document_service.add_document(payload).await?;
    Ok(Json(ApiResponse::new("Document created successfully", document)))
}

pub async fn get_documents(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<DocumentResponse>>>, AppError> {
    let docs = state.document_service.get_documents().await?;

    Ok(Json(ApiResponse::new(
        "Documents fetched successfully",
        docs,
    )))
}

pub async fn get_document_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<DocumentResponse>>, AppError> {
    let doc = state.document_service.get_document_by_id(id).await?;

    Ok(Json(ApiResponse::new("Document fetch success", doc)))
}
