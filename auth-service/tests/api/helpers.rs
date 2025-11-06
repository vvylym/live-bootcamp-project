use sqlx::{
    Connection, Executor, PgConnection, PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use std::cell::Cell;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{
    Application,
    api::{
        AppState,
        utils::constants::{DATABASE_URL, test},
    },
    get_postgres_pool,
    services::data_stores::*,
};
use reqwest::{Client, cookie::Jar};
use uuid::Uuid;

/// A helper struct to spawn and interact with a test instance of our application.
pub struct TestApp {
    /// The address of the running instance of our application.
    pub address: String,

    /// Test Database name
    pub db_name: String,

    /// The cookie jar to store cookies.
    pub cookie_jar: Arc<Jar>,

    pub banned_token_store: Arc<RwLock<HashsetBannedTokenStore>>,

    pub two_fa_code_store: Arc<RwLock<HashmapTwoFACodeStore>>,
    /// The HTTP client to interact with the application.
    pub http_client: Client,

    pub clean_up_called: Cell<bool>,
}

impl TestApp {
    /// Spawns a new instance of our application and returns a `TestApp` instance.
    pub async fn new() -> Self {
        let pg_pool = configure_postgresql().await;
        let db_name = pg_pool
            .connect_options()
            .get_database()
            .unwrap()
            .to_string();
        let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));

        let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
        let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
        let email_client = Arc::new(RwLock::new(MockEmailClient {}));

        let app_state = AppState::new(
            user_store,
            banned_token_store.clone(),
            two_fa_code_store.clone(),
            email_client,
        );

        let app = Application::build(app_state, test::APP_ADDRESS)
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());

        // Create a Reqwest http client instance
        let http_client = Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        // Create new `TestApp` instance and return it
        Self {
            address,
            db_name,
            cookie_jar,
            banned_token_store,
            two_fa_code_store,
            http_client,
            clean_up_called: Cell::new(false),
        }
    }

    /// Sends a GET request to the root endpoint ("/") of the application.
    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    /// Sends a POST request to the "/signup" endpoint of the application.
    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    /// Sends a POST request to the "/login" endpoint of the application.
    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    /// Sends a POST request to the "/logout" endpoint of the application.
    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    /// Sends a POST request to the "/request-2fa" endpoint of the application.
    pub async fn post_verify_2fa<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    /// Sends a POST request to the "/verify-token" endpoint of the application.
    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn clean_up(&self) {
        delete_database(&self.db_name).await;
        self.clean_up_called.set(true);
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        if !self.clean_up_called.get() {
            panic!("TestApp cleanup should be cleared before being dropped!");
        }
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}

async fn configure_postgresql() -> PgPool {
    let postgresql_conn_url = DATABASE_URL.to_owned();

    // We are creating a new database for each test case, and we need to ensure each database has a unique name!
    let db_name = Uuid::new_v4().to_string();

    configure_database(&postgresql_conn_url, &db_name).await;

    let postgresql_conn_url_with_db = format!("{}/{}", postgresql_conn_url, db_name);

    // Create a new connection pool and return it
    get_postgres_pool(&postgresql_conn_url_with_db)
        .await
        .expect("Failed to create Postgres connection pool!")
}

async fn configure_database(db_conn_string: &str, db_name: &str) {
    // Create database connection
    let connection = PgPoolOptions::new()
        .connect(db_conn_string)
        .await
        .expect("Failed to create Postgres connection pool.");

    // Create a new database
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to create database.");

    // Connect to new database
    let db_conn_string = format!("{}/{}", db_conn_string, db_name);

    let connection = PgPoolOptions::new()
        .connect(&db_conn_string)
        .await
        .expect("Failed to create Postgres connection pool.");

    // Run migrations against new database
    sqlx::migrate!()
        .run(&connection)
        .await
        .expect("Failed to migrate the database");
}

async fn delete_database(db_name: &str) {
    let postgresql_conn_url: String = DATABASE_URL.to_owned();

    let connection_options = PgConnectOptions::from_str(&postgresql_conn_url)
        .expect("Failed to parse PostgreSQL connection string");

    let mut connection = PgConnection::connect_with(&connection_options)
        .await
        .expect("Failed to connect to Postgres");

    // Kill any active connections to the database
    connection
        .execute(
            format!(
                r#"
                SELECT pg_terminate_backend(pg_stat_activity.pid)
                FROM pg_stat_activity
                WHERE pg_stat_activity.datname = '{}'
                  AND pid <> pg_backend_pid();
        "#,
                db_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to drop the database.");

    // Drop the database
    connection
        .execute(format!(r#"DROP DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to drop the database.");
}
