use crate::domain::ports::{BannedStore, BannedStoreError};
use std::collections::HashSet;

/// A store for banned tokens using a HashSet.
#[derive(Default, Clone)]
pub struct HashSetBannedStore {
    /// A set of banned tokens.
    banned_tokens: HashSet<String>,
}

impl BannedStore for HashSetBannedStore {
    /// Checks if a token is banned.
    async fn is_banned(&self, token: &str) -> Result<bool, BannedStoreError> {
        Ok(self.banned_tokens.contains(token))
    }

    /// Adds a token to the banned store.
    async fn add_token(&mut self, token: &str) -> Result<(), BannedStoreError> {
        self.banned_tokens.insert(token.to_owned());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_token() {
        let mut banned_store = HashSetBannedStore::default();
        banned_store.add_token("test_token").await.unwrap();
        assert!(banned_store.is_banned("test_token").await.unwrap());
    }

    #[tokio::test]
    async fn test_is_banned() {
        let mut banned_store = HashSetBannedStore::default();
        banned_store.add_token("banned_token").await.unwrap();
        assert!(!banned_store.is_banned("not_banned_token").await.unwrap());
        assert!(banned_store.is_banned("banned_token").await.unwrap());
    }
}
