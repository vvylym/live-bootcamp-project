use crate::domain::{
    models::{Email, Password, User},
    ports::{UserStore, UserStoreError},
};
use std::collections::HashMap;


#[derive(Default, Clone)]
pub struct HashmapUserStore {
    /// A hashmap to store users by their email.
    users: HashMap<String, User>,
}

impl UserStore for HashmapUserStore {
    /// Adds a user to the store.
    async fn add_user(&mut self, user: &User) -> Result<(), UserStoreError> {
        if self.users.contains_key(user.email.as_ref()) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users
            .insert(user.email.as_ref().to_owned(), user.to_owned());
        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        match self.users.get(email.as_ref()) {
            Some(value) => Ok(value.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    /// Validates a user.
    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        match self.users.get(email.as_ref()) {
            Some(user) => {
                if user.password.as_ref() == password.as_ref() {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_email(email: &'static str) -> Email {
        Email::parse(email)
            .map_err(move |_| {
                panic!("Failed to create email");
            })
            .unwrap()
    }

    fn default_password(password: &'static str) -> Password {
        Password::parse(password)
            .map_err(move |_| {
                panic!("Failed to create password");
            })
            .unwrap()
    }

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let email = default_email("user@example.com");
        let password = default_password("password123");
        let user = User::new(email, password, false);
        let result = store.add_user(&user).await;
        assert_eq!(result, Ok(()));
        let result = store.add_user(&user).await;
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let email = default_email("user@example.com");
        let password = default_password("password123");
        let user: User = User::new(email.clone(), password, false);
        let _ = store.add_user(&user).await;
        let result = store.get_user(&email).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user);
        let fake_user = default_email("user2@example.com");
        let result = store.get_user(&fake_user).await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let email = default_email("user@example.com");
        let another_email = default_email("user2@example.com");
        let another_password = default_password("password123");
        let password = default_password("password124");
        let user: User = User::new(email.clone(), password.clone(), false);
        let _ = store.add_user(&user).await;
        let result = store.validate_user(&email, &password).await;
        assert_eq!(result, Ok(()));
        let result = store.validate_user(&email, &another_password).await;
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));
        let result = store.validate_user(&another_email, &password).await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }
}
