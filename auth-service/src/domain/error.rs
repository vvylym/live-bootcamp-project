/// Domain-specific errors for the authentication service.
pub enum AuthAPIError {
    /// Indicates that the provided password is not valid.
    InvalidPassword,
    /// Indicates that the provided email is not valid.
    InvalidEmail,
    /// Indicates that a user with the given email already exists.
    UserAlreadyExists,
    /// Indicates that the provided credentials are invalid.
    InvalidCredentials,
    /// Indicates that an unexpected error occurred.
    UnexpectedError,
}
