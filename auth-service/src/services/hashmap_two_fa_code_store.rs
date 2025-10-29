use std::collections::HashMap;

use crate::domain::{
    ports::{TwoFACodeStore, TwoFACodeStoreError},
    models::{Email, LoginAttemptId, TwoFACode},
};

#[derive(Default, Clone)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

// TODO: implement TwoFACodeStore for HashmapTwoFACodeStore
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
            &mut self,
            email: Email,
            login_attempt_id: LoginAttemptId,
            code: TwoFACode,
        ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        self.codes.remove(&email);
        Ok(())
    }

    async fn get_code(
            &self,
            email: &Email,
        ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.codes.get(&email) {
            Some((login_attempt_id, two_fa_code)) => Ok((login_attempt_id.to_owned(), two_fa_code.to_owned())),
            _ => Err(TwoFACodeStoreError::UnexpectedError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let add_to_store = store.add_code(
            Email::parse("email@example.com").unwrap(), 
            LoginAttemptId::default(),
            TwoFACode::default(),
        );
        assert!(add_to_store.await.is_ok());
    }

    #[tokio::test]
    async fn test_get_code() {
        let valid_email = Email::parse("email@example.com").unwrap();
        let invalid_email  = Email::parse("email2@example.com").unwrap();
        let mut store = HashmapTwoFACodeStore::default();
        let add_to_store = store.add_code(
            valid_email.clone(), 
            LoginAttemptId::default(),
            TwoFACode::default(),
        );
        assert!(add_to_store.await.is_ok());

        let valid_code = store.get_code(&valid_email).await;
        assert!(valid_code.is_ok());

        let invalid_code = store.get_code(&invalid_email).await;
        assert!(invalid_code.is_err());
    }

    #[tokio::test]
    async fn test_remove_code() {
        let valid_email = Email::parse("email@example.com").unwrap();
        let delete_email  = Email::parse("email2@example.com").unwrap();
        let mut store = HashmapTwoFACodeStore::default();
        let _ = store.add_code(
            valid_email.clone(), 
            LoginAttemptId::default(),
            TwoFACode::default(),
        );
        let _ = store.add_code(
            delete_email.clone(), 
            LoginAttemptId::default(),
            TwoFACode::default(),
        );

        let delete_code = store.remove_code(&delete_email).await;
        assert!(delete_code.is_ok());

        let deleted_code = store.get_code(&delete_email).await;
        assert!(deleted_code.is_err());
    }
}