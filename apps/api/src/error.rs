use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub struct AppError {
    pub message: String,
    pub status_code: StatusCode,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = Json(serde_json::json!({
            "message":self.message
        }));

        (self.status_code, body).into_response()
    }
}
