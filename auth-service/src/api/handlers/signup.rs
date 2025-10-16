use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    api::dtos::{ErrorResponse, SignUpRequest, SignUpResponse}, domain::{error::AuthAPIError, models::{Password, Email, User}, ports::UserStore}, AppState
};

#[utoipa::path(
    post,
    path = "/signup",
    description = "Register a new user",
    request_body = SignUpRequest,
    tag = "auth",
    responses(
        (status = 201, description = "User created successfully", body = SignUpResponse, content_type = "application/json"),
        (status = 400, description = "Invalid input", body = ErrorResponse, content_type = "application/json"),
        (status = 409, description = "Email already exists", body = ErrorResponse, content_type = "application/json"),
        (status = 422, description = "Unprocessable content", body = ErrorResponse, content_type = "application/json"),
        (status = 500, description = "Unexpected error", body = ErrorResponse, content_type = "application/json"),
    )
)]
pub async fn handle_signup<S: UserStore>(
    State(state): State<AppState<S>>,
    Json(request): Json<SignUpRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(&request.email)?;
    let password = Password::parse(&request.password)?;

    let user = User::new(
        email, 
        password,
        request.requires_2fa
    );
    let mut user_store = state.user_store.write().await;

    if user_store.get_user(&user.email).await.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    user_store
        .add_user(&user)
        .await
        .map_err(|_| AuthAPIError::UnexpectedError)?;

    let response = Json(SignUpResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}
