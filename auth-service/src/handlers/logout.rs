use axum::{http::StatusCode, response::IntoResponse};

pub async fn handle_logout() -> impl IntoResponse {
    StatusCode::OK.into_response()
}