use auth_service::api::{dtos::ErrorResponse, utils::constants::JWT_COOKIE_NAME};
use reqwest::Url;

use crate::helpers::get_random_email;

use super::helpers::TestApp;

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let password = "password123";

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": password,
        "requires2FA": false
    });

    let _ = app.post_signup(&signup_body).await;

    let login_body = serde_json::json!({
        "email": random_email,
        "password": password,
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);
    
    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);

}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let password = "password123";

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": password,
        "requires2FA": false
    });

    let _ = app.post_signup(&signup_body).await;

    let login_body = serde_json::json!({
        "email": random_email,
        "password": password,
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);
    
    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 400);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to UserBody")
            .error,
        "Missing token"
    );
}

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 400);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to UserBody")
            .error,
        "Missing token"
    );
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 401);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to UserBody")
            .error,
        "Invalid token"
    );
}