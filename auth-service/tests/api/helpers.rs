use auth_service::Application;
use reqwest::Client;

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
        let app = Application::build("127.0.0.1:0")
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
    pub async fn signup(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    /// Sends a POST request to the "/login" endpoint of the application.
    pub async fn login(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    /// Sends a POST request to the "/logout" endpoint of the application.
    pub async fn logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    /// Sends a POST request to the "/request-2fa" endpoint of the application.
    pub async fn verify_2fa(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    /// Sends a POST request to the "/verify-token" endpoint of the application.
    pub async fn verify_token(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}