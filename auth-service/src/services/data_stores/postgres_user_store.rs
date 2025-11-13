use argon2::{
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
    password_hash::SaltString,
};
use color_eyre::eyre::{Context, Result, eyre};
use secrecy::{ExposeSecret, SecretString};
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
    #[tracing::instrument(name = "Adding user to PostgreSQL", skip_all)]
    async fn add_user(&mut self, user: &User) -> Result<(), UserStoreError> {
        let password_hash = 
            compute_password_hash(user.password.as_ref().to_owned())
                .await
                .map_err(UserStoreError::UnexpectedError)?;

        sqlx::query!(
            r#"
            INSERT INTO users (email, password_hash, requires_2fa)
            VALUES ($1, $2, $3)
            "#,
            &user.email.as_ref().expose_secret(),
            &password_hash.expose_secret(),
            user.requires_2fa
        )
        .execute(&self.pool)
        .await
        .map_err(|e| UserStoreError::UnexpectedError(e.into()))?;

        Ok(())
    }

    #[tracing::instrument(name = "Get user from PostgreSQL", skip_all)]
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        sqlx::query!(
            r#"
            SELECT email, password_hash as password, requires_2fa 
            FROM users 
            WHERE email = $1
            "#,
            &email.as_ref().expose_secret(),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| UserStoreError::UnexpectedError(e.into()))?
        .map(|row| {
            Ok(User {
                email: Email::parse(row.email.into())
                    .map_err(|e| UserStoreError::UnexpectedError(eyre!(e)))?,
                password: Password::parse(row.password.into())
                    .map_err(|e| UserStoreError::UnexpectedError(eyre!(e)))?,
                requires_2fa: row.requires_2fa,
            })
        })
        .ok_or(UserStoreError::UserNotFound)?
    }

    #[tracing::instrument(name = "Validate user credentials in PostgreSQL", skip_all)]
    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        let user = self.get_user(email).await?;
        let expected_password_hash = user.password.as_ref();
        let password_candidate = password.as_ref();
        verify_password_hash(
            expected_password_hash.to_owned(),
            password_candidate.to_owned(),
        )
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
#[tracing::instrument(name = "Verify password hash", skip_all)]
async fn verify_password_hash(
    expected_password_hash: SecretString,
    password_candidate: SecretString,
) -> Result<()> {
    let current_span: tracing::Span = tracing::Span::current();

    let result = tokio::task::spawn_blocking(move || {
        // This code block ensures that the operations within the closure are executed within the context of the current span.
        // This is especially useful for tracing operations that are performed in a different thread or task, such as within tokio::task::spawn_blocking.
        current_span.in_scope(|| {
            // New!
            let expected_password_hash: PasswordHash<'_> =
                PasswordHash::new(
                    &expected_password_hash.expose_secret()
                )?;

            Argon2::default()
                .verify_password(
                    password_candidate.expose_secret().as_bytes(), 
                    &expected_password_hash
                )
                .wrap_err("failed to verify password hash")
        })
    })
    .await;

    result?
}

// Helper function to hash passwords before persisting them in the database.
// TODO: Hashing is a CPU-intensive operation. To avoid blocking
// other async tasks, update this function to perform hashing on a
// separate thread pool using tokio::task::spawn_blocking. Note that you
// will need to update the input parameters to be String types instead of &str
#[tracing::instrument(name = "Computing password hash", skip_all)]
async fn compute_password_hash(password: SecretString) -> Result<SecretString> {
    // This line retrieves the current span from the tracing context.
    // The span represents the execution context for the compute_password_hash function.
    let current_span: tracing::Span = tracing::Span::current();
    
    let result = tokio::task::spawn_blocking(move || {
        // This code block ensures that the operations within the closure are executed within the context of the current span.
        // This is especially useful for tracing operations that are performed in a different thread or task, such as within tokio::task::spawn_blocking.
        current_span.in_scope(|| {
            // New!
            let salt: SaltString =
                SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
            let password_hash = Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Params::new(15000, 2, 1, None)?,
            )
            .hash_password(password.expose_secret().as_bytes(), &salt)?
            .to_string();

            Ok(password_hash.into())
        })
    })
    .await;

    result?
}
