use crate::error::AppError;
use crate::models::documents::{DocumentDTO, DocumentRequest, DocumentResponse, DocumentStatus};
use crate::repositories::documents::DocumentsRepository;
use axum::http::StatusCode;
use uuid::Uuid;

pub struct DocumentService {
    pub repository: DocumentsRepository,
}

impl DocumentService {
    pub async fn add_document(&self, data: DocumentRequest) -> Result<DocumentResponse, AppError> {
        let document_dto = DocumentDTO {
            id: Uuid::new_v4(),
            file_name: data.file_name,
            content_type: data.content_type,
            status: DocumentStatus::Queued,
        };
        let doc = self.repository.create(document_dto).map_err(|e| AppError {
            message: e,
            status_code: StatusCode::BAD_REQUEST,
        })?;

        let response_doc = DocumentResponse {
            id: doc.id,
            file_name: doc.filename,
            content_type: doc.content_type,
        };

        Ok(response_doc)
    }
}
