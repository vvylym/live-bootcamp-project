use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

/// Defines the sign-up request model.
#[derive(Debug, Deserialize, ToSchema, Validate)]
#[schema(example = json!({
    "email": "email@example.com",
    "password": "secret",
    "requires2FA": true
}))]
pub struct SignUpRequest {
    /// The user's email address.
    #[validate(email)]
    pub email: String,
    /// The user's password.
    #[validate(length(min = 4))]
    pub password: String,
    /// Indicates if two-factor authentication is required.
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

/// Defines the login request model.
#[derive(Debug, Deserialize, ToSchema, Validate)]
#[schema(example = json!({
    "email": "email@example.com",
    "password": "secret"
}))]
pub struct LoginRequest {
    /// The user's email address.
    //#[validate(email)]
    pub email: String,
    /// The user's password.
    //#[validate(length(min = 4))]
    pub password: String,
}

/// Defines the login request model.
#[derive(Debug, Deserialize, ToSchema, Validate)]
#[schema(example = json!({
    "email": "email@example.com",
    "loginAttemptId": "attempt-123-123",
    "2FACode": "1234"
}))]
pub struct Verify2faRequest {
    /// The user's email address.]
    #[validate(email)]
    pub email: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
    /// The user's password.
    #[serde(rename = "2FACode")]
    #[validate(length(min = 6))]
    pub _2fa_code: String,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
#[schema(example = json!({
    "token": "123e4567-e89b-12d3-a456-426614174000"
}))]
pub struct VerifyTokenRequest {
    /// The user's email address.
    pub token: String,
}
