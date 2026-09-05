use crate::constants::MAX_PROCESS_JOB_ATTEMPTS;
use crate::error::AppError;
use crate::jobs::channel::Job;
use domain::documents::DocumentStatus;
use crate::repositories::documents::DocumentsRepository;
use crate::storage::local::store_multipart_file;
use axum::extract::Multipart;
use axum::http::StatusCode;
use tokio::sync::mpsc;
use uuid::Uuid;
use crate::models::documents::{DocumentDTO,DocumentResponse};

pub struct DocumentService {
    pub repository: DocumentsRepository,
    pub tx: mpsc::Sender<Job>,
}

impl DocumentService {
    pub async fn add_document(&self, multipart: Multipart) -> Result<DocumentResponse, AppError> {
        let data = store_multipart_file(multipart).await?;
        let document_dto = DocumentDTO {
            id: data.id,
            file_name: data.file_name,
            content_type: data.content_type,
            status: DocumentStatus::Queued,
            path: data.path,
        };

        let job = Job {
            id: document_dto.id,
            attempts: 0,
            max_attempts: MAX_PROCESS_JOB_ATTEMPTS,
        };
        let doc = self.repository.create(document_dto).await?;

        self.tx.send(job).await.map_err(|e| AppError {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            errors: None,
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
        let d = self.repository.get().await?;

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
        let document = self.repository.get_by_id(id).await?;

        let document = document.ok_or(AppError {
            message: "Document not found".into(),
            status_code: StatusCode::NOT_FOUND,
            errors: None,
        })?;

        let doc = DocumentResponse {
            id: document.id,
            file_name: document.file_name,
            content_type: document.content_type,
        };
        Ok(doc)
    }
}
