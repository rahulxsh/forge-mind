use crate::error::AppError;
use crate::jobs::channel::Job;
use crate::models::documents::{DocumentDTO, DocumentRequest, DocumentResponse, DocumentStatus};
use crate::repositories::documents::DocumentsRepository;
use axum::http::StatusCode;
use tokio::sync::mpsc;
use uuid::Uuid;

pub struct DocumentService {
    pub repository: DocumentsRepository,
    pub tx: mpsc::Sender<Job>,
}

impl DocumentService {
    pub async fn add_document(&self, data: DocumentRequest) -> Result<DocumentResponse, AppError> {
        let document_dto = DocumentDTO {
            id: Uuid::new_v4(),
            file_name: data.file_name,
            content_type: data.content_type,
            status: DocumentStatus::Queued,
        };

        let job = Job {
            id: document_dto.id,
        };

        self.tx.send(job).await.map_err(|e| AppError {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;
        let doc = self
            .repository
            .create(document_dto)
            .await
            .map_err(|e| AppError {
                message: e,
                status_code: StatusCode::BAD_REQUEST,
            })?;

        let response_doc = DocumentResponse {
            id: doc.id,
            file_name: doc.file_name,
            content_type: doc.content_type,
        };

        Ok(response_doc)
    }

    pub async fn get_documents(&self) -> Result<Vec<DocumentResponse>, AppError> {
        let mut docs = Vec::new();
        let d = self.repository.get().await.map_err(|e| AppError {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

        for i in d.into_iter() {
            docs.push(DocumentResponse {
                id: i.id,
                file_name: i.file_name,
                content_type: i.content_type,
            })
        }

        Ok(docs)
    }

    pub async fn get_document_by_id(&self, id: Uuid) -> Result<DocumentResponse, AppError> {
        let document = self.repository.get_by_id(id).await.map_err(|e| AppError {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;
        let doc = DocumentResponse {
            id: document.id,
            file_name: document.file_name,
            content_type: document.content_type,
        };
        Ok(doc)
    }
}
