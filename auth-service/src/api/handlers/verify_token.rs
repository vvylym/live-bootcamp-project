use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    api::{
        dtos::{ErrorResponse, VerifyTokenRequest},
        utils::auth::validate_token,
    },
    domain::{error::AuthAPIError, ports::{BannedStore, UserStore}}, AppState,
};

#[utoipa::path(
    post,
    path = "/verify-token",
    description = "Verify JWT",
    request_body = VerifyTokenRequest,
    tag = "auth",
    responses(
        (status = 200, description = "Token is valid"),
        (status = 401, description = "JWT is not valid", body = ErrorResponse, content_type = "application/json"),
        (status = 422, description = "Unprocessable content", body = ErrorResponse, content_type = "application/json"),
        (status = 500, description = "Unexpected error", body = ErrorResponse, content_type = "application/json"),
    )
)]
pub async fn handle_verify_token<S: UserStore, B: BannedStore>(
    State(state): State<AppState<S, B>>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let token = request.token.to_owned();

    let banned_store = state.banned_store.read().await;
    if banned_store.is_banned(&token).await.unwrap() {
        return Err(AuthAPIError::InvalidToken);
    }

    validate_token(&token)
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;

    Ok(StatusCode::OK.into_response())
}
