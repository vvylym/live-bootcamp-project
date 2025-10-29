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
        ports::{BannedStore, TwoFACodeStore, UserStore},
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
pub async fn handle_login<S: UserStore, B: BannedStore, T: TwoFACodeStore>(
    State(state): State<AppState<S, B, T>>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let email = Email::parse(&request.email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
        Password::parse(&request.password).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = state.user_store.write().await;

    if user_store.validate_user(&email, &password).await.is_err() {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return Err(AuthAPIError::IncorrectCredentials),
    };

     // Handle request based on user's 2FA configuration
    match user.requires_2fa {
        true => handle_2fa(&email, &state, jar).await,
        false => handle_no_2fa(&user.email, jar).await,
    }
}

// New!
async fn handle_2fa<S: UserStore, B: BannedStore, T: TwoFACodeStore>(
    email: &Email, 
    state: &AppState<S, B, T>, // New!
    jar: CookieJar,
) -> Result<(
    CookieJar,
    (StatusCode, Json<LoginResponse>)), AuthAPIError> {

    // First, we must generate a new random login attempt ID and 2FA code
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    let mut two_fa_store = state.two_fa_store.write().await;
    
    if two_fa_store.add_code(email.to_owned(), login_attempt_id.clone(), two_fa_code).await.is_err() {
        return Err(AuthAPIError::UnexpectedError);
    }

    Ok((
        jar, 
        (
            StatusCode::PARTIAL_CONTENT, 
            Json(LoginResponse::TwoFactorAuth(MFARequiredResponse { 
                message: "2FA required".to_owned(), 
                login_attempt_id: login_attempt_id.as_ref().to_string() }))
        )
    ))
}

// New!
async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> Result<(CookieJar,(StatusCode, Json<LoginResponse>)), AuthAPIError> {

    let auth_cookie = generate_auth_cookie(&email).map_err(|_| AuthAPIError::UnexpectedError)?;

    let updated_jar = jar.add(auth_cookie);

    Ok((updated_jar, (StatusCode::OK, Json(LoginResponse::RegularAuth))))
}

use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(MFARequiredResponse),
}
