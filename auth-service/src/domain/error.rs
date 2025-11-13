use color_eyre::eyre::Report;
use thiserror::Error;

/// Domain-specific errors for the authentication service.
#[derive(Debug, Error)]
pub enum AuthAPIError {
    /// Indicates that the provided password is not valid.
    #[error("Invalid password")]
    InvalidPassword,
    /// Indicates that the provided email is not valid.
    #[error("Invalid email address")]
    InvalidEmail,
    /// Indicates that a user with the given email already exists.
    #[error("User already exists")]
    UserAlreadyExists,
    /// Indicates that the provided credentials are invalid.
    #[error("Invalid credentials")]
    InvalidCredentials,
    /// Indicates that the provided credentials are incorrect.
    #[error("Incorrect credentials")]
    IncorrectCredentials,
    /// Indicates that the provided token is missing.
    #[error("Missing token")]
    MissingToken,
    /// Indicates that the provided token is invalid.
    #[error("Invalid token")]
    InvalidToken,
    /// Indicates that an unexpected error occurred.
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}
