use crate::models::documents::{Document, DocumentDTO, DocumentStatus};
use sqlx::PgPool;
use tracing::info;
use uuid::Uuid;

#[derive(Clone)]
pub struct DocumentsRepository {
    pub pool: PgPool,
}

impl DocumentsRepository {
    pub async fn create(&self, data: DocumentDTO) -> Result<Document, sqlx::Error> {
        let document = sqlx::query_as!(
            Document,
            r#"
                INSERT INTO documents
                    (id, file_name, content_type, status, path)
                VALUES
                    ($1, $2, $3, $4, $5)
                RETURNING
                    id,
                    file_name,
                    content_type,
                    status AS "status: DocumentStatus",
                    path
            "#,
            data.id,
            data.file_name,
            data.content_type,
            data.status as DocumentStatus,
            data.path
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(document)
    }

    pub async fn get(&self) -> Result<Vec<Document>, sqlx::Error> {
        let documents = sqlx::query_as!(
            Document,
            r#"
                SELECT id, file_name, content_type, status AS "status:DocumentStatus", path
                FROM documents
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(documents)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Document>, sqlx::Error> {
        let document = sqlx::query_as!(
            Document,
            r#"
                SELECT id, file_name, content_type, status AS "status:DocumentStatus", path
                FROM documents
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(document)
    }

    pub async fn update_status(&self, id: Uuid, status: DocumentStatus) -> Result<(), sqlx::Error> {
        info!(%id, ?status, "Updating document status");
        sqlx::query!(
            r#"
            UPDATE documents
            SET status = $1
            WHERE id = $2
            "#,
            status as DocumentStatus,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
