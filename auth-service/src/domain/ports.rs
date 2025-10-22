use super::models::{Email, Password, User};
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
    fn is_banned(&self, token: &str) -> impl Future<Output = Result<bool, BannedStoreError>> + Send;

    /// Adds a token to the banned store.
    fn add_token(&mut self, token: &str) -> impl Future<Output = Result<(), BannedStoreError>> + Send;
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
