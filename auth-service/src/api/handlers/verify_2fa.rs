use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{AppState, api::{dtos::{ErrorResponse, Verify2faRequest}, utils::auth::generate_auth_cookie}, domain::{error::AuthAPIError, models::{Email, LoginAttemptId, TwoFACode}, ports::{BannedStore, EmailClient, TwoFACodeStore, UserStore}}};

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
pub async fn handle_verify_2fa<S: UserStore, B: BannedStore, T: TwoFACodeStore, E: EmailClient>(
    State(state): State<AppState<S, B, T, E>>,
    jar: CookieJar,
    Json(request): Json<Verify2faRequest>,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let email = 
        Email::parse(&request.email)
            .map_err(|_| AuthAPIError::InvalidCredentials)?;
    let login_attempt_id = 
        LoginAttemptId::parse(request.login_attempt_id)
            .map_err(|_| AuthAPIError::InvalidCredentials)?;
    let two_fa_code = 
        TwoFACode::parse(request._2fa_code)
            .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let two_fa_code_store = state.two_fa_store.write().await;
    // Call `two_fa_code_store.get_code`. If the call fails
    // return a `AuthAPIError::IncorrectCredentials`.
    let code = two_fa_code_store.get_code(&email)
        .await.map_err(|_| AuthAPIError::IncorrectCredentials)?;
    
    let code_tuple = (login_attempt_id, two_fa_code); 
    // TODO: Validate that the `login_attempt_id` and `two_fa_code`
    // in the request body matches values in the `code_tuple`. 
    // If not, return a `AuthAPIError::IncorrectCredentials`.
    if code != code_tuple {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    let auth_cookie = generate_auth_cookie(&email).map_err(|_| AuthAPIError::UnexpectedError)?;

    let updated_jar = jar.add(auth_cookie);


    Ok((updated_jar, StatusCode::OK.into_response()))
}
