use std::sync::Arc;

use color_eyre::eyre::Context;
use redis::{Commands, Connection};
use secrecy::{ExposeSecret, SecretString};
use tokio::sync::RwLock;

use crate::{
    api::utils::constants::TOKEN_TTL_SECONDS,
    domain::ports::{BannedTokenStore, BannedTokenStoreError},
};

#[derive(Clone)]
pub struct RedisBannedTokenStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisBannedTokenStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

impl RedisBannedTokenStore {
    pub async fn reset_all(&mut self) {
        let mut conn = self.conn.write().await;
        let _: () = conn.flushall().unwrap_or_else(|_| {
            println!("Failed to reset all");
        });
    }
}

impl BannedTokenStore for RedisBannedTokenStore {
    async fn add_token(&mut self, token: &SecretString) -> Result<(), BannedTokenStoreError> {
        let token_key = get_key(token);

        let value = true;

        let ttl: u64 = TOKEN_TTL_SECONDS
            .try_into()
            .wrap_err("failed to cast TOKEN_TTL_SECONDS to u64") // New!
            .map_err(BannedTokenStoreError::UnexpectedError)?; // Updated!

        let _: () = self
            .conn
            .write()
            .await
            .set_ex(&token_key, value, ttl)
            .wrap_err("failed to set banned token in Redis") // New!
            .map_err(BannedTokenStoreError::UnexpectedError)?; // Updated!

        Ok(())
    }

    async fn contains_token(&self, token: &SecretString) -> Result<bool, BannedTokenStoreError> {
        let token_key = get_key(token);

        let is_banned: bool = self
            .conn
            .write()
            .await
            .exists(&token_key)
            .wrap_err("failed to check if token exists in Redis") // New!
            .map_err(BannedTokenStoreError::UnexpectedError)?; // Updated!

        Ok(is_banned)
    }
}

// We are using a key prefix to prevent collisions and organize data!
const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";

fn get_key(token: &SecretString) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token.expose_secret())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::utils::constants::REDIS_HOST_NAME;
    use crate::get_redis_client;

    fn create_test_store() -> RedisBannedTokenStore {
        let redis_host = REDIS_HOST_NAME.to_owned();
        let client = get_redis_client(redis_host).expect("Failed to get Redis client");
        let connection = client
            .get_connection()
            .expect("Failed to get Redis connection");
        RedisBannedTokenStore::new(Arc::new(RwLock::new(connection)))
    }

    #[tokio::test]
    async fn test_redis_banned_token_store() {
        // 1. Initialize Redis store
        let mut store = create_test_store();

        // 2. Add a single token
        let token = "test_token_single";
        let result = store.add_token(&token.into()).await;
        assert!(result.is_ok(), "Failed to add token");

        // 3. Verify that the added key exists
        let contains = store.contains_token(&token.into()).await;
        assert!(contains.is_ok(), "Failed to check if token exists");
        assert!(contains.unwrap(), "Token should exist after being added");

        // 4. Verify that a non-existing key does not exist
        let non_existing_token = "non_existing_token";
        let non_existing = store.contains_token(&non_existing_token.into()).await;
        assert!(
            non_existing.is_ok(),
            "Failed to check if non-existing token exists"
        );
        assert!(
            !non_existing.unwrap(),
            "Non-existing token should not exist"
        );

        // 5. Call flushall (reset_all)
        store.reset_all().await;

        // 6. Verify that the single key added no longer exists
        let contains_after_flush = store.contains_token(&token.into()).await;
        assert!(
            contains_after_flush.is_ok(),
            "Failed to check if token exists after flush"
        );
        assert!(
            !contains_after_flush.unwrap(),
            "Token should not exist after flushall"
        );
    }
}
