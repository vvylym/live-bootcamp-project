use super::models::*;
use std::future::Future;

/// A trait for a user store.
pub trait UserStore: Send + Sync + Clone + 'static {
    /// Adds a user to the store.
    fn add_user(&mut self, user: &User) -> impl Future<Output = Result<(), UserStoreError>> + Send;

    /// Gets a user from the store.
    fn get_user(&self, email: &Email) -> impl Future<Output = Result<User, UserStoreError>> + Send;

    /// Validates a user.
    fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> impl Future<Output = Result<(), UserStoreError>> + Send;
}

/// A trait for a banned store.
pub trait BannedStore: Send + Sync + Clone + 'static {
    /// Checks if an email is banned.
    fn is_banned(&self, token: &str)
    -> impl Future<Output = Result<bool, BannedStoreError>> + Send;

    /// Adds a token to the banned store.
    fn add_token(
        &mut self,
        token: &str,
    ) -> impl Future<Output = Result<(), BannedStoreError>> + Send;
}

/// An error that can occur when interacting with the user store.
#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    /// Indicates that a user with the given email already exists.
    UserAlreadyExists,
    /// Indicates that a user with the given email was not found.
    UserNotFound,
    /// Indicates that the provided credentials are invalid.
    InvalidCredentials,
    /// Indicates that an unexpected error occurred.
    UnexpectedError,
}

/// An error that can occur when interacting with the banned store.
#[derive(Debug, PartialEq)]
pub enum BannedStoreError {
    /// Indicates that an unexpected error occurred.
    UnexpectedError,
}

// This trait represents the interface all concrete 2FA code stores should implement
pub trait TwoFACodeStore: Send + Sync + Clone + 'static {
    fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> impl Future<Output = Result<(), TwoFACodeStoreError>> + Send;
    fn remove_code(
        &mut self,
        email: &Email,
    ) -> impl Future<Output = Result<(), TwoFACodeStoreError>> + Send;
    fn get_code(
        &self,
        email: &Email,
    ) -> impl Future<Output = Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>> + Send;
}

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttemptIdNotFound,
    UnexpectedError,
}

///
pub trait EmailClient: Send + Sync + Clone + 'static {
    fn send_email(
        &self,
        recipient: &Email,
        subject: &str,
        content: &str,
    ) -> impl Future<Output = Result<(), String>> + Send;
}
