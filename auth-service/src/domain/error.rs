/// Domain-specific errors for the authentication service.
#[derive(Debug)]
pub enum AuthAPIError {
    /// Indicates that the provided password is not valid.
    InvalidPassword,
    /// Indicates that the provided email is not valid.
    InvalidEmail,
    /// Indicates that a user with the given email already exists.
    UserAlreadyExists,
    /// Indicates that the provided credentials are invalid.
    InvalidCredentials,
    /// Indicates that the provided credentials are incorrect.
    IncorrectCredentials,
    /// Indicates that the provided token is missing.
    MissingToken,
    /// Indicates that the provided token is invalid.
    InvalidToken,
    /// Indicates that an unexpected error occurred.
    UnexpectedError,
}
