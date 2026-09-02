use crate::constants::ALLOWED_EXTENSIONS;
use crate::error::AppError;
use crate::jobs::channel::Job;
use crate::models::documents::{DocumentDTO, DocumentResponse, DocumentStatus};
use crate::repositories::documents::DocumentsRepository;
use axum::extract::Multipart;
use axum::http::StatusCode;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;
use tracing::info;
use uuid::Uuid;

pub struct DocumentService {
    pub repository: DocumentsRepository,
    pub tx: mpsc::Sender<Job>,
}

impl DocumentService {
    pub async fn add_document(
        &self,
        mut multipart: Multipart,
    ) -> Result<DocumentResponse, AppError> {
        let id = Uuid::new_v4();

        let mut document_dto = DocumentDTO {
            id,
            file_name: "hello.pdf".into(),
            content_type: "application/pdf".into(),
            status: DocumentStatus::Queued,
            path: format!("{}.{}", id, "pdf"),
        };

        while let Some(mut field) = multipart.next_field().await.map_err(|_e| AppError {
            message: "Parse error".into(),
            status_code: StatusCode::BAD_REQUEST,
            errors: None,
        })? {
            let file_name = field.file_name().unwrap_or("unnamed_file").to_string();

            let extension = Path::new(&file_name)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("");

            if !ALLOWED_EXTENSIONS.contains(&extension) {
                return Err(AppError {
                    message: format!("Extension '.{}' is not allowed.", extension),
                    status_code: StatusCode::BAD_REQUEST,
                    errors: None,
                });
            }

            let content_type = field.content_type().ok_or(AppError {
                message: "Missing content type".into(),
                status_code: StatusCode::BAD_REQUEST,
                errors: None,
            })?;
            document_dto.content_type = content_type.to_string();
            let path = format!("{}.{}", id, extension);

            let path_e = Path::new("./uploads").join(path);

            let mut dest_file = match File::create(path_e.clone()).await {
                Ok(f) => f,
                Err(e) => {
                    info!("File Creation Error: {} ", e);
                    return Err(AppError {
                        message: "File upload failed".into(),
                        status_code: StatusCode::INTERNAL_SERVER_ERROR,
                        errors: None,
                    });
                }
            };

            while let Some(chunk) = field.chunk().await.unwrap() {
                if let Err(e) = dest_file.write_all(&chunk).await {
                    info!("File Write Error:{}", e);
                    return Err(AppError {
                        message: "Server Error: Failed to upload file".into(),
                        status_code: StatusCode::INTERNAL_SERVER_ERROR,
                        errors: None,
                    });
                }
            }

            document_dto.file_name = file_name;
            document_dto.path = path_e.to_str().unwrap().to_string();
        }

        let job = Job {
            id: document_dto.id,
        };

        self.tx.send(job).await.map_err(|e| AppError {
            message: e.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            errors: None,
        })?;

        let doc = self.repository.create(document_dto).await?;

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
