use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{api::models::{ErrorResponse, SignUpRequest, SignUpResponse}, domain::user::User, services::hashmap_user_store::UserStoreError, AppState};

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
pub async fn handle_signup(
    State(state): State<AppState>,
    Json(_input): Json<SignUpRequest>,
) -> impl IntoResponse {
    let user = User::new(
        _input.email,
        _input.password,
        _input.requires_2fa,
    );
    let mut user_store = state
        .user_store
        .write()
        .await;
    
    user_store
        .add_user(user)
        .map(
            |_| (StatusCode::CREATED, Json(SignUpResponse{ message: "User created successfully!".to_string() }))
        )
        .map_err(
            |e| match e {
                UserStoreError::UserAlreadyExists => (StatusCode::CONFLICT, Json(ErrorResponse{ error: "Ok".to_string() })),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse{ error: "Ok".to_string() })),
            }
        )
}
