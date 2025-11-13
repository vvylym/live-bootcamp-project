use secrecy::SecretString;
use serde::Deserialize;
use utoipa::ToSchema;

/// Defines the sign-up request model.
#[derive(Debug, Deserialize, ToSchema)]
#[schema(example = json!({
    "email": "email@example.com",
    "password": "secret123",
    "requires2FA": true
}))]
pub struct SignUpRequest {
    /// The user's email address.
    #[schema(value_type = String)]
    pub email: SecretString,
    /// The user's password.
    #[schema(value_type = String)]
    pub password: SecretString,
    /// Indicates if two-factor authentication is required.
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

/// Defines the login request model.
#[derive(Debug, Deserialize, ToSchema)]
#[schema(example = json!({
    "email": "email@example.com",
    "password": "secret123",
}))]
pub struct LoginRequest {
    /// The user's email address.
    #[schema(value_type = String)]
    pub email: SecretString,
    /// The user's password.
    #[schema(value_type = String)]
    pub password: SecretString,
}

/// Defines the login request model.
#[derive(Debug, Deserialize, ToSchema)]
#[schema(example = json!({
    "email": "email@example.com",
    "loginAttemptId": "attempt-123-123",
    "2FACode": "123456",
}))]
pub struct Verify2faRequest {
    /// The user's email address.
    #[schema(value_type = String)]
    pub email: SecretString,
    #[serde(rename = "loginAttemptId")]
    #[schema(value_type = String)]
    pub login_attempt_id: SecretString,
    /// The user's password.
    #[serde(rename = "2FACode")]
    #[schema(value_type = String)]
    pub _2fa_code: SecretString,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example = json!({
    "token": "123e4567-e89b-12d3-a456-426614174000",
}))]
pub struct VerifyTokenRequest {
    /// The user's email address.
    #[schema(value_type = String)]
    pub token: SecretString,
}
