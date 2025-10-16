pub mod dtos;
pub mod handlers;
pub mod routes;

use routes::api_routes;

use axum::{Router, serve::Serve};
use std::sync::Arc;
use tokio::{net::TcpListener, sync::RwLock};

use crate::{domain::ports::UserStore, services::hashmap_user_store::HashmapUserStore};

// Using a type alias to improve readability!
pub type UserStoreType = Arc<RwLock<HashmapUserStore>>;

#[derive(Clone)]
pub struct AppState<S: UserStore> {
    pub user_store: Arc<RwLock<S>>,
}

impl<S> AppState<S>
where
    S: UserStore,
{
    pub fn new(user_store: Arc<RwLock<S>>) -> Self {
        Self { user_store }
    }
}

// This struct encapsulates our application-related logic.
pub struct Application {
    /// The axum server instance.
    server: Serve<TcpListener, Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    /// Builds a new instance of the `Application`.
    pub async fn build<S: UserStore>(
        app_state: AppState<S>,
        address: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Move the Router definition from `main.rs` to here.
        // Also, remove the `hello` route.
        // We don't need it at this point!
        let router = api_routes(app_state);

        let listener = TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Self { server, address })
    }

    /// Runs the application server.
    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}
