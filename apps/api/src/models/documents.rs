use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use domain::documents::DocumentStatus;

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
