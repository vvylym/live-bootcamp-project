use axum::{http::StatusCode, response::IntoResponse};

#[tracing::instrument(name = "Root", skip_all)]
pub async fn handle_root() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
