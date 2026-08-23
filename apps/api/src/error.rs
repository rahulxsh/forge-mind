use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use sqlx::Error;
use validator::ValidationErrors;

pub struct AppError {
    pub message: String,
    pub status_code: StatusCode,
    pub errors: Option<Vec<FieldError>>,
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    message: String,
    data: Option<()>,
    errors: Option<Vec<FieldError>>,
}

#[derive(Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = Json(ErrorResponse {
            success: false,
            message: self.message,
            data: None,
            errors: self.errors,
        });

        (self.status_code, body).into_response()
    }
}

impl From<ValidationErrors> for AppError {
    fn from(error: ValidationErrors) -> Self {
        let errors = error
            .field_errors()
            .into_iter()
            .map(|(field, errors)| FieldError {
                field: field.to_string(),
                message: errors
                    .first()
                    .and_then(|e| e.message.clone())
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| "Invalid value".to_string()),
            })
            .collect();

        Self {
            message: "Validation failed".into(),
            status_code: StatusCode::BAD_REQUEST,
            errors: Some(errors),
        }
    }
}


impl From<sqlx::Error> for AppError {
    fn from(error: Error) -> Self {
        tracing::error!(error = ?error, "Database error");

        Self {
            message: "Database operation failed".into(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            errors: None,
        }
    }
}