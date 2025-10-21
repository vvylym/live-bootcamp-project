use axum::{
    Json, Router,
    http::Method,
    response::Html,
    routing::{get, post},
};
use tower_http::{cors::CorsLayer, services::ServeDir};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

use crate::{api::AppState, domain::ports::UserStore};

use super::handlers::*;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Auth Service API",
        description = "Auth Service OpenAPI Specification. This document describes all of the operations available through the Auth Service API.", 
        contact(
            name = "vvylym",
            url = "https://github.com/vvylym/live-bootcamp-project",
            email = "235853469+vvylym@users.noreply.github.com",
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/license/mit",
        ),
        version = "0.1.0",
    ),
    servers(
        (url = "http://localhost:3000", description = "Local development server"),
    ),
    paths(
        handle_signup,
        handle_login,
        handle_logout,
        handle_verify_2fa,
        handle_verify_token,
        openapi,
    ),
    components(
        schemas(
            super::dtos::SignUpRequest,
            super::dtos::LoginRequest,
            super::dtos::Verify2faRequest,
            super::dtos::VerifyTokenRequest,
            super::dtos::SignUpResponse,
            super::dtos::MFARequiredResponse,
            super::dtos::ErrorResponse
        ),
    ),
    tags(
        (name = "auth", description = "Authentication endpoints."),
        (name = "docs", description = "Documentation endpoints."),
    ),
)]
struct ApiDoc;

pub fn api_routes<S: UserStore>(app_state: AppState<S>) -> Router {
    let allowed_origins = [
        "http://localhost:8000".parse().unwrap(),
        // TODO: Replace [YOUR_DROPLET_IP] with your Droplet IP address
        "http://[YOUR_DROPLET_IP]:8000".parse().unwrap(),
    ];

    let cors = CorsLayer::new()
        // Allow GET and POST requests
        .allow_methods([Method::GET, Method::POST])
        // Allow cookies to be included in requests
        .allow_credentials(true)
        .allow_origin(allowed_origins);

    Router::new()
        .route("/", get(handle_root))
        .route("/login", post(handle_login))
        .route("/logout", post(handle_logout))
        .route("/signup", post(handle_signup))
        .route("/verify-2fa", post(handle_verify_2fa))
        .route("/verify-token", post(handle_verify_token))
        .route("/api-docs/openapi.json", get(openapi))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/api-docs"))
        .fallback_service(ServeDir::new("auth-service/assets"))
        .with_state(app_state)
        .layer(cors)
}

#[utoipa::path(
    get,
    path = "/api-docs/openapi.json",
    description = "OPENAPI Json specifications file",
    tag = "docs",
    responses(
        (status = 200, description = "JSON file", body = ())
    )
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

/// Serves the root HTML page
async fn handle_root() -> Html<&'static str> {
    Html(include_str!("../../assets/index.html"))
}
