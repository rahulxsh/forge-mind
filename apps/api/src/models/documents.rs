use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct DocumentDTO {
    pub id: Uuid,
    pub file_name: String,
    pub content_type: String,
    pub status: DocumentStatus,
}

#[derive(Deserialize)]
pub struct DocumentRequest {
    pub file_name: String,
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
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "document_status", rename_all = "lowercase")]
pub enum DocumentStatus {
    Queued,
    Processing,
    Processed,
    Failed,
}
