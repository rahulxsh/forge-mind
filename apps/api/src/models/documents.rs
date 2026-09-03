use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub struct DocumentDTO {
    pub id: Uuid,
    pub file_name: String,
    pub content_type: String,
    pub status: DocumentStatus,
    pub path: String,
}

#[derive(Deserialize, Validate)]
pub struct DocumentRequest {
    #[validate(length(min = 1, message = "Invalid Filename"))]
    pub file_name: String,

    #[validate(length(min = 1, message = "Invalid content type"))]
    pub content_type: String,
}

#[derive(Serialize)]
pub struct DocumentResponse {
    pub id: Uuid,
    pub file_name: String,
    pub content_type: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Document {
    pub id: Uuid,
    pub file_name: String,
    pub content_type: String,
    pub status: DocumentStatus,
    pub path: String,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "document_status", rename_all = "lowercase")]
pub enum DocumentStatus {
    Queued,
    Processing,
    Processed,
    Failed,
}
