use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    AppState,
    api::{
        dtos::{ErrorResponse, LoginRequest, MFARequiredResponse},
        utils::auth::generate_auth_cookie,
    },
    domain::{
        error::AuthAPIError,
        models::{Email, LoginAttemptId, Password, TwoFACode},
        ports::{BannedTokenStore, EmailClient, TwoFACodeStore, UserStore},
    },
};

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
#[tracing::instrument(name = "Login", skip_all)]
pub async fn handle_login<S: UserStore, B: BannedTokenStore, T: TwoFACodeStore, E: EmailClient>(
    State(state): State<AppState<S, B, T, E>>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let password = match Password::parse(request.password) {
        Ok(password) => password,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let user_store = &state.user_store.read().await;

    if user_store.validate_user(&email, &password).await.is_err() {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    match user.requires_2fa {
        true => handle_2fa(&user.email, &state, jar).await,
        false => handle_no_2fa(&user.email, jar).await,
    }
}

// New!
#[tracing::instrument(name = "Handle 2FA", skip_all)]
async fn handle_2fa<S: UserStore, B: BannedTokenStore, T: TwoFACodeStore, E: EmailClient>(
    email: &Email,
    state: &AppState<S, B, T, E>,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    if let Err(e) = state
        .two_fa_store
        .write()
        .await
        .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
        .await
    {
        return (jar, Err(AuthAPIError::UnexpectedError(e.into())));
    }

    if let Err(e) = state
        .email_client
        .read()
        .await
        .send_email(email, "2FA Code", two_fa_code.as_ref())
        .await
    {
        return (jar, Err(AuthAPIError::UnexpectedError(e)));
    }

    let response = Json(LoginResponse::TwoFactorAuth(MFARequiredResponse {
        message: "2FA required".to_owned(),
        login_attempt_id: login_attempt_id.as_ref().to_owned(),
    }));

    (jar, Ok((StatusCode::PARTIAL_CONTENT, response)))
}

// New!
#[tracing::instrument(name = "Handle No 2FA", skip_all)]
async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let auth_cookie = match generate_auth_cookie(email) {
        Ok(cookie) => cookie,
        Err(e) => return (jar, Err(AuthAPIError::UnexpectedError(e.into()))),
    };

    let updated_jar = jar.add(auth_cookie);

    (
        updated_jar,
        Ok((StatusCode::OK, Json(LoginResponse::RegularAuth))),
    )
}

use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(MFARequiredResponse),
}
