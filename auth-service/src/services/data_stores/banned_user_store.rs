use crate::domain::ports::{BannedTokenStore, BannedTokenStoreError};
use std::collections::HashSet;

/// A store for banned tokens using a HashSet.
#[derive(Default, Clone)]
pub struct HashsetBannedTokenStore {
    /// A set of banned tokens.
    tokens: HashSet<String>,
}

impl BannedTokenStore for HashsetBannedTokenStore {
    /// Checks if a token is banned.
    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        Ok(self.tokens.contains(token))
    }

    /// Adds a token to the banned store.
    async fn add_token(&mut self, token: &str) -> Result<(), BannedTokenStoreError> {
        self.tokens.insert(token.to_owned());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_token() {
        let mut store = HashsetBannedTokenStore::default();
        let token = "test_token".to_owned();

        let result = store.add_token(&token).await;

        assert!(result.is_ok());
        assert!(store.tokens.contains(&token));
    }

    #[tokio::test]
    async fn test_contains_token() {
        let mut store = HashsetBannedTokenStore::default();
        let token = "test_token".to_owned();
        store.tokens.insert(token.clone());

        let result = store.contains_token(&token).await;

        assert!(result.unwrap());
    }
}
