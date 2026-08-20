use crate::models::documents::{Document, DocumentDTO, DocumentStatus};
use sqlx::PgPool;
use uuid::Uuid;

pub struct DocumentsRepository {
    pub pool: PgPool,
}

impl DocumentsRepository {
    pub async fn create(&self, data: DocumentDTO) -> Result<Document, String> {
        Ok(Document {
            id: data.id,
            file_name: data.file_name,
            content_type: data.content_type,
            status: data.status,
        })
    }

    pub async fn get(&self) -> Result<Vec<Document>, String> {
        let docs = vec![
            Document {
                id: Uuid::new_v4(),
                file_name: "hello.txt".into(),
                content_type: "application/txt".into(),
                status: DocumentStatus::Processing,
            },
            Document {
                id: Uuid::new_v4(),
                file_name: "github.pdf".into(),
                content_type: "application/pdf".into(),
                status: DocumentStatus::Failed,
            },
            Document {
                id: Uuid::new_v4(),
                file_name: "transactions.csv".into(),
                content_type: "application/csv".into(),
                status: DocumentStatus::Queued,
            },
        ];

        Ok(docs)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Document, String> {
        let d = Document {
            id,
            file_name: "transactions.csv".into(),
            content_type: "application/csv".into(),
            status: DocumentStatus::Queued,
        };

        Ok(d)
    }
}
