use crate::models::documents::{Document, DocumentDTO};

pub struct DocumentsRepository;

impl DocumentsRepository {
    pub fn create(&self, data: DocumentDTO) -> Result<Document, String> {
        Ok(Document {
            id: data.id,
            filename: data.file_name,
            content_type: data.content_type,
            status: data.status,
        })
    }
}
