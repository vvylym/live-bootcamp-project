use crate::api::dtos::ErrorResponse;
use axum::{http::StatusCode, response::IntoResponse};

#[utoipa::path(
    post,
    path = "/logout",
    description = "Logout user",
    // TODO: Add parameters in cookie with schema string, required with the name jwt
    tag = "auth",
    responses(
        (status = 200, description = "Logout successful", headers(("x-set-cookie" = String, description = "jwt=; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; SameSite=Lax; Secure; Path=/")),),
        (status = 400, description = "Invalid input", body = ErrorResponse, content_type = "application/json"),
        (status = 401, description = "JWT is not valid", body = ErrorResponse, content_type = "application/json"),
        (status = 500, description = "Unexpected error", body = ErrorResponse, content_type = "application/json"),
    )
)]
pub async fn handle_logout() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
