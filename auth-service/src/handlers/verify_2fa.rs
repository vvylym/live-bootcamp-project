use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::models::{Verify2faRequest, ErrorResponse};

#[utoipa::path(
    post,
    path = "/verify-2fa",
    description = "Verify 2FA token",
    request_body = Verify2faRequest,
    tag = "auth",
    responses(
        (status = 200, description = "Login successful", 
            headers(("x-set-cookie" = String, description = "jwt=your_token; HttpOnly; SameSite=Lax; Secure; Path=/")),
        ),
        (status = 400, description = "Invalid input", body = ErrorResponse, content_type = "application/json"),
        (status = 401, description = "Authentication failed", body = ErrorResponse, content_type = "application/json"),
        (status = 422, description = "Unprocessable content", body = ErrorResponse, content_type = "application/json"),
        (status = 500, description = "Unexpected error", body = ErrorResponse, content_type = "application/json"),
    )
)]
pub async fn handle_verify_2fa(Json(_input): Json<Verify2faRequest>) -> impl IntoResponse {
    StatusCode::OK.into_response()
}
