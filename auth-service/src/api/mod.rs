pub mod dtos;
pub mod handlers;
pub mod routes;
pub mod utils;

use routes::api_routes;

use axum::{Router, serve::Serve};
use std::sync::Arc;
use tokio::{net::TcpListener, sync::RwLock};

use crate::domain::ports::{BannedStore, EmailClient, TwoFACodeStore, UserStore};

#[derive(Clone)]
pub struct AppState<S: UserStore, B: BannedStore, T: TwoFACodeStore, E: EmailClient> {
    pub user_store: Arc<RwLock<S>>,
    pub banned_store: Arc<RwLock<B>>,
    pub two_fa_store: Arc<RwLock<T>>,
    pub email_client: Arc<RwLock<E>>,
}

impl<S, B, T, E> AppState<S, B, T, E>
where
    S: UserStore,
    B: BannedStore,
    T: TwoFACodeStore,
    E: EmailClient,
{
    pub fn new(
        user_store: Arc<RwLock<S>>,
        banned_store: Arc<RwLock<B>>,
        two_fa_store: Arc<RwLock<T>>,
        email_client: Arc<RwLock<E>>,
    ) -> Self {
        Self {
            user_store,
            banned_store,
            two_fa_store,
            email_client,
        }
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
    pub async fn build<S: UserStore, B: BannedStore, T: TwoFACodeStore, E: EmailClient>(
        app_state: AppState<S, B, T, E>,
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
