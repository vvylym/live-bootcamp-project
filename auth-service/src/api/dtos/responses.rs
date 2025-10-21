use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::error::AuthAPIError;

/// Defines the response model for successful sign-up.
#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema)]
#[schema(example = json!({
    "message": "User created successfully."
}))]
pub struct SignUpResponse {
    pub message: String,
}

/// Defines the response model for successful login requiring 2FA.
#[derive(Debug, Serialize, ToSchema)]
#[schema(example = json!({
    "message": "MFA required.",
    "loginAttemptId": "123e4567-e89b-12d3-a456-426614174000"
}))]
pub struct MFARequiredResponse {
    /// The success message.
    pub message: String,
    /// The unique identifier for the login attempt.
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}

/// Defines the error response model.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "error": "Invalid credentials."
}))]
pub struct ErrorResponse {
    /// The error message.
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::InvalidPassword => (StatusCode::BAD_REQUEST, "Invalid password"),
            AuthAPIError::InvalidEmail => (StatusCode::BAD_REQUEST, "Invalid email address"),
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Incorrect credentials"),
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing token"),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}
