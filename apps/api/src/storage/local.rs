use crate::constants::ALLOWED_EXTENSIONS;
use crate::error::AppError;
use axum::extract::Multipart;
use axum::http::StatusCode;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::info;
use uuid::Uuid;

pub struct StoreMultipartFileResponse {
    pub id: Uuid,
    pub file_name: String,
    pub content_type: String,
    pub path: String,
}

pub async fn store_multipart_file(
    mut multipart: Multipart,
) -> Result<StoreMultipartFileResponse, AppError> {
    let id = Uuid::new_v4();
    let mut data = StoreMultipartFileResponse {
        id,
        file_name: "".into(),
        content_type: "".into(),
        path: "".into(),
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
        data.content_type = content_type.to_string();
        let path = format!("{}.{}", id, extension);

        let upload_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("uploads");
        let path_e = upload_dir.join(path);

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

        while let Some(chunk) = field.chunk().await.map_err(|e| {
            info!("File Read Error: {}", e);

            AppError {
                message: "Failed to read uploaded file".into(),
                status_code: StatusCode::BAD_REQUEST,
                errors: None,
            }
        })? {
            if let Err(e) = dest_file.write_all(&chunk).await {
                info!("File Write Error:{}", e);
                return Err(AppError {
                    message: "Server Error: Failed to upload file".into(),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    errors: None,
                });
            }
        }

        data.file_name = file_name;
        data.path = path_e.to_str().unwrap().to_string();
    }

    Ok(data)
}
