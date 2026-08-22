use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub struct AppError {
    pub message: String,
    pub status_code: StatusCode,
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    message: String,
    data: Option<()>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = Json(ErrorResponse{
            success: false,
            message: self.message,
            data: None,
        });

        (self.status_code, body).into_response()
    }
}
