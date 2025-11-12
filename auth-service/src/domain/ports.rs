use color_eyre::eyre::Report;
use rand::Rng;
use std::future::Future;
use thiserror::Error;

use super::models::*;


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
pub trait BannedTokenStore: Send + Sync + Clone + 'static {
    /// Checks if an email is banned.
    fn contains_token(
        &self,
        token: &str,
    ) -> impl Future<Output = Result<bool, BannedTokenStoreError>> + Send;

    /// Adds a token to the banned store.
    fn add_token(
        &mut self,
        token: &str,
    ) -> impl Future<Output = Result<(), BannedTokenStoreError>> + Send;
}

/// An error that can occur when interacting with the user store.
#[derive(Debug, Error)]
pub enum UserStoreError {
    /// Indicates that a user with the given email already exists.
    #[error("User already exists")]
    UserAlreadyExists,
    /// Indicates that a user with the given email was not found.
    #[error("Unexpected error")]
    UserNotFound,
    /// Indicates that the provided credentials are invalid.
    #[error("Invalid credentials")]
    InvalidCredentials,
    /// Indicates that an unexpected error occurred.
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

impl PartialEq for UserStoreError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::UserAlreadyExists, Self::UserAlreadyExists)
                | (Self::UserNotFound, Self::UserNotFound)
                | (Self::InvalidCredentials, Self::InvalidCredentials)
                | (Self::UnexpectedError(_), Self::UnexpectedError(_))
        )
    }
}

/// An error that can occur when interacting with the banned store.
#[derive(Debug, PartialEq)]
pub enum BannedTokenStoreError {
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

/// This trait represent the interface for email clients.
pub trait EmailClient: Send + Sync + Clone + 'static {
    fn send_email(
        &self,
        recipient: &Email,
        subject: &str,
        content: &str,
    ) -> impl Future<Output = Result<(), String>> + Send;
}
