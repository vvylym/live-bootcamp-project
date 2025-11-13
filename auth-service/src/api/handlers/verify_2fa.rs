use axum::{Json, extract::State, response::IntoResponse};
use axum_extra::extract::CookieJar;
use color_eyre::eyre::eyre;
use secrecy::ExposeSecret;

use crate::{
    AppState,
    api::{
        dtos::{ErrorResponse, Verify2faRequest},
        utils::auth::generate_auth_cookie,
    },
    domain::{
        error::AuthAPIError,
        models::{Email, LoginAttemptId, TwoFACode},
        ports::{BannedTokenStore, EmailClient, TwoFACodeStore, UserStore},
    },
};

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
#[tracing::instrument(name = "Verify 2FA", skip_all)]
pub async fn handle_verify_2fa<
    S: UserStore,
    B: BannedTokenStore,
    T: TwoFACodeStore,
    E: EmailClient,
>(
    State(state): State<AppState<S, B, T, E>>,
    jar: CookieJar,
    Json(request): Json<Verify2faRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let login_attempt_id = match LoginAttemptId::parse(request.login_attempt_id) {
        Ok(login_attempt_id) => login_attempt_id,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let two_fa_code = match TwoFACode::parse(request._2fa_code) {
        Ok(two_fa_code) => two_fa_code,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let mut two_fa_code_store = state.two_fa_store.write().await;

    let code_tuple = match two_fa_code_store.get_code(&email).await {
        Ok(code_tuple) => code_tuple,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    if !code_tuple.0.eq(&login_attempt_id) || !code_tuple.1.eq(&two_fa_code) {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    if let Err(e) = two_fa_code_store.remove_code(&email).await {
        return (jar, Err(AuthAPIError::UnexpectedError(eyre!(e))));
    }

    let cookie = match generate_auth_cookie(&email) {
        Ok(cookie) => cookie,
        Err(e) => return (jar, Err(AuthAPIError::UnexpectedError(eyre!(e)))),
    };

    let updated_jar = jar.add(cookie);

    (updated_jar, Ok(()))
}
