use axum::{http::StatusCode, response::IntoResponse};

pub async fn handle_login() -> impl IntoResponse {
    StatusCode::OK.into_response()
}