use crate::{
    AppState,
    api::{
        dtos::ErrorResponse,
        utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
    },
    domain::{
        error::AuthAPIError,
        ports::{BannedTokenStore, EmailClient, TwoFACodeStore, UserStore},
    },
};
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{CookieJar, cookie::Cookie};

#[utoipa::path(
    post,
    path = "/logout",
    description = "Logout user",
    tag = "auth",
    responses(
        (status = 200, description = "Logout successful", headers(("x-set-cookie" = String, description = "jwt=; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; SameSite=Lax; Secure; Path=/")),),
        (status = 400, description = "Invalid input", body = ErrorResponse, content_type = "application/json"),
        (status = 401, description = "JWT is not valid", body = ErrorResponse, content_type = "application/json"),
        (status = 500, description = "Unexpected error", body = ErrorResponse, content_type = "application/json"),
    )
)]
#[tracing::instrument(name = "Logout", skip_all)]
pub async fn handle_logout<S: UserStore, B: BannedTokenStore, T: TwoFACodeStore, E: EmailClient>(
    State(state): State<AppState<S, B, T, E>>,
    jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie,
        None => return (jar, Err(AuthAPIError::MissingToken)),
    };

    // Validate token
    let token = cookie.value().to_owned();
    let _ = match validate_token(&token, state.banned_token_store.clone()).await {
        Ok(claims) => claims,
        Err(_) => return (jar, Err(AuthAPIError::InvalidToken)),
    };

    // Add token to banned list
    if let Err(e) = state
        .banned_token_store
        .write()
        .await
        .add_token(&token)
        .await
    {
        return (jar, Err(AuthAPIError::UnexpectedError(e.into())));
    }

    // Remove jwt cookie
    let jar = jar.remove(Cookie::from(JWT_COOKIE_NAME));

    (jar, Ok(StatusCode::OK))
}
