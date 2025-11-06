use std::error::Error;

use argon2::{
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
    password_hash::SaltString,
};

use sqlx::PgPool;

use crate::domain::{
    models::{Email, Password, User},
    ports::{UserStore, UserStoreError},
};

#[derive(Clone)]
pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserStore for PostgresUserStore {
    // TODO: Implement all required methods. Note that you will need to make SQL queries against our PostgreSQL instance inside these methods.

    async fn add_user(&mut self, user: &User) -> Result<(), UserStoreError> {
        let password_hash = compute_password_hash(user.password.as_ref().to_string())
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?;

        sqlx::query!(
            "INSERT INTO users (email, password_hash, requires_2fa) VALUES ($1, $2, $3)",
            user.email.as_ref(),
            password_hash,
            user.requires_2fa
        )
        .execute(&self.pool)
        .await
        .map_err(|_| UserStoreError::UnexpectedError)?;

        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let record = sqlx::query!(
            "SELECT email, password_hash as password, requires_2fa FROM users WHERE email = $1",
            email.as_ref()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => UserStoreError::UserNotFound,
            _ => UserStoreError::UnexpectedError,
        })?;
        let user = User {
            email: Email::parse(&record.email).map_err(|_| UserStoreError::UnexpectedError)?,
            password: Password::parse(&record.password)
                .map_err(|_| UserStoreError::UnexpectedError)?,
            requires_2fa: record.requires_2fa,
        };
        Ok(user)
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        let user = self.get_user(email).await?;
        let expected_password_hash = user.password.as_ref();
        let password_candidate = password.as_ref();
        verify_password_hash(expected_password_hash, password_candidate)
            .await
            .map_err(|_| UserStoreError::InvalidCredentials)?;
        Ok(())
    }
}

// Helper function to verify if a given password matches an expected hash
// TODO: Hashing is a CPU-intensive operation. To avoid blocking
// other async tasks, update this function to perform hashing on a
// separate thread pool using tokio::task::spawn_blocking. Note that you
// will need to update the input parameters to be String types instead of &str
async fn verify_password_hash(
    expected_password_hash: &str,
    password_candidate: &str,
) -> Result<(), Box<dyn Error>> {
    let expected_password_hash: PasswordHash<'_> = PasswordHash::new(expected_password_hash)?;

    Argon2::default()
        .verify_password(password_candidate.as_bytes(), &expected_password_hash)
        .map_err(|e| e.into())
}

// Helper function to hash passwords before persisting them in the database.
// TODO: Hashing is a CPU-intensive operation. To avoid blocking
// other async tasks, update this function to perform hashing on a
// separate thread pool using tokio::task::spawn_blocking. Note that you
// will need to update the input parameters to be String types instead of &str
async fn compute_password_hash(password: String) -> Result<String, Box<dyn Error>> {
    let salt: SaltString = SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).map_err(|e| e.to_string())?,
    )
    .hash_password(password.as_bytes(), &salt)
    .map_err(|e| e.to_string())?
    .to_string();

    Ok(password_hash)
}
