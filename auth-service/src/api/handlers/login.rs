use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;

use crate::{api::{dtos::{ErrorResponse, LoginRequest, MFARequiredResponse}, utils::auth::generate_auth_cookie}, domain::{error::AuthAPIError, models::{Email, Password}, ports::UserStore}, AppState};

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
pub async fn handle_login<S: UserStore>(
    State(state): State<AppState<S>>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    
    let email = Email::parse(&request.email)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(&request.password)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = state.user_store.write().await;

    user_store.validate_user(&email, &password)
        .await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    let auth_cookie = generate_auth_cookie(&email)
        .map_err(|_| AuthAPIError::UnexpectedError)?;
    
    let updated_jar = jar.add(auth_cookie);

    Ok((updated_jar, (StatusCode::OK.into_response())))
}
