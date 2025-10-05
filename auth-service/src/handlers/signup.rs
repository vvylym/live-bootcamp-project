use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::models::{SignUpRequest, SignUpResponse, ErrorResponse};

#[utoipa::path(
    post,
    path = "/signup",
    description = "Register a new user",
    request_body = SignUpRequest,
    tag = "auth",
    responses(
        (status = 201, description = "User created successfully", body = SignUpResponse, content_type = "application/json"),
        (status = 400, description = "Invalid input", body = ErrorResponse, content_type = "application/json"),
        (status = 409, description = "Email already exists", body = ErrorResponse, content_type = "application/json"),
        (status = 422, description = "Unprocessable content", body = ErrorResponse, content_type = "application/json"),
        (status = 500, description = "Unexpected error", body = ErrorResponse, content_type = "application/json"),
    )
)]
pub async fn handle_signup(Json(_input): Json<SignUpRequest>) -> impl IntoResponse {
    StatusCode::OK.into_response()
}
