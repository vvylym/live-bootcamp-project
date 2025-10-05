use axum::{http::StatusCode, response::IntoResponse};

pub async fn handle_signup() -> impl IntoResponse {
    StatusCode::OK.into_response()
}