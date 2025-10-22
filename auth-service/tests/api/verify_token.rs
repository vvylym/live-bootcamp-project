use auth_service::api::{dtos::ErrorResponse, utils::constants::JWT_COOKIE_NAME};

use super::helpers::*;

#[tokio::test]
async fn should_return_200_valid_token() {
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

    let token = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .unwrap()
        .value()
        .to_string();

    let verify_token_body = serde_json::json!({
        "token": token,
    });

    let response = app.post_verify_token(&verify_token_body).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_401_if_banned_token() {
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

    let token = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .unwrap()
        .value()
        .to_string();

    let verify_token_body = serde_json::json!({
        "token": token,
    });

    let _ = app.post_logout().await;

    let response = app.post_verify_token(&verify_token_body).await;

    assert_eq!(response.status().as_u16(), 401);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Invalid token"
    );

}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    let invalid_token_body = serde_json::json!({
        "token": "invalid_token",
    });

    let response = app.post_verify_token(&invalid_token_body).await;

    assert_eq!(response.status().as_u16(), 401);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Invalid token"
    );
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "email": true,
            "password": random_email
        }),
        serde_json::json!({
            "email": 123,
        }),
        serde_json::json!({
            "password": 123,
        }),
        serde_json::json!({
            "email": random_email,
            "password": true
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_verify_token(test_case).await; // call `post_signup`
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}
