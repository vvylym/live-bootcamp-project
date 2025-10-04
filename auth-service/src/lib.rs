//! Auth Service library
//! 
//! This module contains the core application logic for the Auth Service.
//! It defines the `Application` struct, which encapsulates the server setup and routing.
//! 
//! The `Application` struct provides methods to build and run the service.
//! It uses the `axum` framework for routing and handling HTTP requests.
use axum::{serve::Serve, response::Html, routing::get, Router};
use tower_http::services::ServeDir;

// This struct encapsulates our application-related logic.
pub struct Application {
    /// The axum server instance.
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    /// Builds a new instance of the `Application`.
    pub async fn build(address: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Move the Router definition from `main.rs` to here.
        // Also, remove the `hello` route.
        // We don't need it at this point!
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/hello", get(hello_handler));

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Self {
            server,
            address,
        })
    }

    /// Runs the application server.
    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}


pub async fn hello_handler() -> Html<&'static str> {
    Html("<h1>Auth Service</h1><br/><p>More coming soon...</p>")
}
