use crate::{api::{dtos::ErrorResponse, utils::{auth::validate_token, constants::JWT_COOKIE_NAME}}, domain::error::AuthAPIError};
use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::{cookie::Cookie, CookieJar};

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
pub async fn handle_logout(jar: CookieJar) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let cookie = jar.get(JWT_COOKIE_NAME)
        .ok_or(AuthAPIError::MissingToken)?;

    let token = cookie.value().to_owned();

    validate_token(&token)
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;

    let jar = jar.clone().remove(Cookie::new(JWT_COOKIE_NAME, cookie.value().to_owned()));

    Ok((jar, StatusCode::OK.into_response()))
}
