use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::api::models::{ErrorResponse, LoginRequest, MFARequiredResponse};

#[utoipa::path(
    post,
    path = "/login",
    description = "Authenticate user and return JWT",
    request_body = LoginRequest,
    tag = "auth",
    responses(
        (status = 200, description = "Login successful", 
            headers(("x-set-cookie" = String, description = "jwt=your_token; HttpOnly; SameSite=Lax; Secure; Path=/")),
        ),
        (status = 206, description = "Login requires 2FA", body = MFARequiredResponse, content_type = "application/json"),
        (status = 400, description = "Invalid input", body = ErrorResponse, content_type = "application/json"),
        (status = 401, description = "Authentication failed", body = ErrorResponse, content_type = "application/json"),
        (status = 422, description = "Unprocessable content", body = ErrorResponse, content_type = "application/json"),
        (status = 500, description = "Unexpected error", body = ErrorResponse, content_type = "application/json"),
    )
)]
pub async fn handle_login(Json(_input): Json<LoginRequest>) -> impl IntoResponse {
    StatusCode::OK.into_response()
}
