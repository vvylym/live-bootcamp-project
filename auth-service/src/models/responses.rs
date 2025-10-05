use serde::Serialize;
use utoipa::ToSchema;

/// Defines the response model for successful sign-up.
#[derive(Debug, Serialize, ToSchema)]
#[schema(example = json!({
    "message": "User created successfully."
}))]
pub struct SignUpResponse {
    message: String,
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
#[derive(Debug, Serialize, ToSchema)]
#[schema(example = json!({
    "error": "Invalid credentials."
}))]
pub struct ErrorResponse {
    /// The error message.
    pub error: String,
}
