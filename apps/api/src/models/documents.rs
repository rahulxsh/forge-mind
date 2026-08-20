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

pub struct Document {
    pub id: Uuid,
    pub filename: String,
    pub content_type: String,
    pub status: DocumentStatus,
}

pub enum DocumentStatus {
    Queued,
    Processing,
    Processed,
    Failed,
}
