use auth_service::{
    api::{AppState, UserStoreType},
    domain::user::User,
    Application,
};
use reqwest::Client;
use uuid::Uuid;

/// A helper struct to spawn and interact with a test instance of our application.
pub struct TestApp {
    /// The address of the running instance of our application.
    pub address: String,
    /// The HTTP client to interact with the application.
    pub http_client: Client,
}

impl TestApp {
    /// Spawns a new instance of our application and returns a `TestApp` instance.
    pub async fn new() -> Self {
        let user_store = UserStoreType::default();
        user_store
            .write()
            .await
            .add_user(User::new(
                "default@user.com".to_string(),
                "secret".to_string(),
                false,
            ))
            .expect("Failed to add default user");
        let app_state = AppState::new(user_store);

        let app = Application::build(app_state, "127.0.0.1:0")
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        // Create a Reqwest http client instance
        let http_client = Client::builder().build().unwrap();

        // Create new `TestApp` instance and return it
        Self {
            address,
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
