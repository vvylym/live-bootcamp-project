use std::collections::HashMap;

use crate::domain::{
    data_stores::{UserStore, UserStoreError},
    user::User,
};

// TODO: Create a new struct called `HashmapUserStore` containing a `users` field
// which stores a `HashMap`` of email `String`s mapped to `User` objects.
// Derive the `Default` trait for `HashmapUserStore`.
#[derive(Default, Clone)]
pub struct HashmapUserStore {
    /// A hashmap to store users by their email.
    users: HashMap<String, User>,
}

impl UserStore for HashmapUserStore {
    /// Adds a user to the store.
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    // TODO: Implement a public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    /// This function should return a `Result` type containing either a
    /// `User` object or a `UserStoreError`.
    /// Return `UserStoreError::UserNotFound` if the user can not be found.
    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(value) => Ok(value.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    /// Validates a user.
    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if user.password == password {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new(
            "user@example.com".to_string(),
            "password123".to_string(),
            false,
        );
        let result = store.add_user(user.clone()).await;
        assert_eq!(result, Ok(()));
        let result = store.add_user(user).await;
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new(
            "user@example.com".to_string(),
            "password123".to_string(),
            false,
        );
        let _ = store.add_user(user.clone()).await;
        let result = store.get_user("user@example.com").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user);
        let result = store.get_user("user2@example.com").await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new(
            "user@example.com".to_string(),
            "password123".to_string(),
            false,
        );
        let _ = store.add_user(user.clone()).await;
        let result = store.validate_user("user@example.com", "password123").await;
        assert_eq!(result, Ok(()));
        let result = store.validate_user("user@example.com", "password124").await;
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));
        let result = store
            .validate_user("user3@example.com", "password123")
            .await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }
}
