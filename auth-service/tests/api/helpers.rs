use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{
    Application, BannedStoreType, TwoFACodeStoreType, api::{AppState, utils::constants::test}, services::{banned_user_store::HashSetBannedStore, hashmap_two_fa_code_store::HashmapTwoFACodeStore, hashmap_user_store::HashmapUserStore}
};
use reqwest::{Client, cookie::Jar};
use uuid::Uuid;

/// A helper struct to spawn and interact with a test instance of our application.
pub struct TestApp {
    /// The address of the running instance of our application.
    pub address: String,
    /// The cookie jar to store cookies.
    pub cookie_jar: Arc<Jar>,
    
    pub banned_token_store: BannedStoreType,

    pub two_fa_code_store: TwoFACodeStoreType,
    /// The HTTP client to interact with the application.
    pub http_client: Client,
}

impl TestApp {
    /// Spawns a new instance of our application and returns a `TestApp` instance.
    pub async fn new() -> Self {
        let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
        let banned_token_store = Arc::new(RwLock::new(HashSetBannedStore::default()));
        let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));

        let app_state = AppState::new(user_store, banned_token_store.clone(), two_fa_code_store.clone());

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
            cookie_jar,
            banned_token_store,
            two_fa_code_store,
            http_client,
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
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
