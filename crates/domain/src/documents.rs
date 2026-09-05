use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Document {
    pub id: Uuid,
    pub file_name: String,
    pub content_type: String,
    pub status: DocumentStatus,
    pub path: String,
    pub attempts: i32,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "document_status", rename_all = "lowercase")]
pub enum DocumentStatus {
    Queued,
    Processing,
    Processed,
    Failed,
}
