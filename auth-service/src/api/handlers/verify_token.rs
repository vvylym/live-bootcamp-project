use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    AppState,
    api::{
        dtos::{ErrorResponse, VerifyTokenRequest},
        utils::auth::validate_token,
    },
    domain::{
        error::AuthAPIError,
        ports::{BannedTokenStore, EmailClient, TwoFACodeStore, UserStore},
    },
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
#[tracing::instrument(name = "Verify Token", skip_all)]
pub async fn handle_verify_token<
    S: UserStore,
    B: BannedTokenStore,
    T: TwoFACodeStore,
    E: EmailClient,
>(
    State(state): State<AppState<S, B, T, E>>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    match validate_token(&request.token, state.banned_token_store.clone()).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(AuthAPIError::InvalidToken),
    }
}
