use axum::{routing::post, Router};

use crate::handlers::*;

pub fn api_routes() -> Router {
    Router::new()
        .route("/login", post(handle_login))
        .route("/logout", post(handle_logout))
        .route("/signup", post(handle_signup))
        .route("/verify-2fa", post(handle_verify_2fa))
        .route("/verify-token", post(handle_verify_token))
}