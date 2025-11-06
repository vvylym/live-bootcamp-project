use std::sync::Arc;

use redis::{Commands, Connection};
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
    async fn add_token(&mut self, token: &str) -> Result<(), BannedTokenStoreError> {
        // 1. Create a new key using the get_key helper function.
        // 2. Call the set_ex command on the Redis connection to set a new key/value pair with an expiration time (TTL).
        // The value should simply be a `true` (boolean value).
        // The expiration time should be set to TOKEN_TTL_SECONDS.
        // NOTE: The TTL is expected to be a u64 so you will have to cast TOKEN_TTL_SECONDS to a u64.
        // Return BannedTokenStoreError::UnexpectedError if casting fails or the call to set_ex fails.
        let key = get_key(token);
        let mut conn = self.conn.write().await;
        // Redis stores strings, so we store "1" to represent true
        let _: () = conn
            .set_ex(key, "true".to_string(), TOKEN_TTL_SECONDS as u64)
            .map_err(|_| BannedTokenStoreError::UnexpectedError)?;
        Ok(())
    }

    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        // Check if the token exists by calling the exists method on the Redis connection
        let key = get_key(token);

        let mut conn = self.conn.write().await;
        match conn.exists(key) {
            Ok(exists) => Ok(exists),
            Err(_) => Err(BannedTokenStoreError::UnexpectedError),
        }
    }
}

// We are using a key prefix to prevent collisions and organize data!
const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";

fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
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
        let result = store.add_token(token).await;
        assert!(result.is_ok(), "Failed to add token");
        
        // 3. Verify that the added key exists
        let contains = store.contains_token(token).await;
        assert!(contains.is_ok(), "Failed to check if token exists");
        assert!(contains.unwrap(), "Token should exist after being added");
        
        // 4. Verify that a non-existing key does not exist
        let non_existing_token = "non_existing_token";
        let non_existing = store.contains_token(non_existing_token).await;
        assert!(non_existing.is_ok(), "Failed to check if non-existing token exists");
        assert!(!non_existing.unwrap(), "Non-existing token should not exist");
        
        // 5. Call flushall (reset_all)
        store.reset_all().await;
        
        // 6. Verify that the single key added no longer exists
        let contains_after_flush = store.contains_token(token).await;
        assert!(contains_after_flush.is_ok(), "Failed to check if token exists after flush");
        assert!(!contains_after_flush.unwrap(), "Token should not exist after flushall");
    }
}
