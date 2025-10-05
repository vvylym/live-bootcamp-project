use axum::{http::StatusCode, response::IntoResponse};

pub async fn handle_root() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
