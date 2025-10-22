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

pub use api::{
    AppState, Application, UserStoreType, BannedStoreType,
    utils::constants::{prod, test},
};
