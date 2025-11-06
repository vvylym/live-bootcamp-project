//! Auth Service library
//!
//! This module contains the core application logic for the Auth Service.
//! It defines the `Application` struct, which encapsulates the server setup and routing.
//!
//! The `Application` struct provides methods to build and run the service.
//! It uses the `axum` framework for routing and handling HTTP requests.
pub mod api;
pub mod domain;
pub mod services;

use domain::ports::{BannedTokenStore, EmailClient, TwoFACodeStore, UserStore};
use std::sync::Arc;
use tokio::sync::RwLock;

pub use api::{
    AppState, Application,
    utils::constants::{prod, test},
};

// Using a type alias to improve readability!
pub type UserStoreType = Arc<RwLock<dyn UserStore>>;

pub type BannedTokenStoreType = Arc<RwLock<dyn BannedTokenStore>>;

pub type TwoFACodeStoreType = Arc<RwLock<dyn TwoFACodeStore>>;

pub type EmailClientType = Arc<RwLock<dyn EmailClient>>;
