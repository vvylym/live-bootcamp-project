use auth_service::api::utils::constants::DATABASE_URL;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{AppState, Application, get_postgres_pool, prod};

use auth_service::services::data_stores::*;

#[tokio::main]
async fn main() {
    let pg_pool = configure_postgresql().await;

    let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));

    let banned_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let two_fa_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
    let email_client = Arc::new(RwLock::new(MockEmailClient {}));

    let app_state = AppState::new(user_store, banned_store, two_fa_store, email_client);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}

async fn configure_postgresql() -> PgPool {
    // Create a new database connection pool
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    // Run database migrations against our test database!
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}
