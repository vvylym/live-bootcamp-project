use auth_service::{
    domain::{
        models::{Email, LoginAttemptId, TwoFACode},
        ports::TwoFACodeStore,
    },
    api::{
        dtos::{MFARequiredResponse, ErrorResponse},
        utils::constants::JWT_COOKIE_NAME, // New!
        
    },
};

use super::helpers::*;

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
        let response = app.post_verify_2fa(test_case).await; // call `post_signup`
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}


#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "email": "emailexample",
            "loginAttemptId": uuid::Uuid::new_v4().to_string(),
            "2FACode": "123456"
        }),
        serde_json::json!({
            "email": random_email,
            "loginAttemptId": "login-attempt-id",
            "2FACode": "123456"
        }),
        serde_json::json!({
            "email": random_email,
            "loginAttemptId": uuid::Uuid::new_v4().to_string(),
            "2FACode": "1234"
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_verify_2fa(test_case).await; // call `post_signup`
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let random_uuid = uuid::Uuid::new_v4().to_string();

    let response = app.post_verify_2fa(
        &serde_json::json!({
            "email": random_email,
            "loginAttemptId": random_uuid,
            "2FACode": "123456"
        })
    ).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    // Call login twice. Then, attempt to call verify-fa with the 2FA code from the first login requet. This should fail. 
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let random_password = "password123";

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": random_password,
    });    
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    let (login_attempt_id, two_fa_code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(&random_email).unwrap())
        .await
        .unwrap();

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    let response = app.post_verify_2fa(
        &serde_json::json!({
            "email": random_email,
            "loginAttemptId": login_attempt_id.as_ref(),
            "2FACode": two_fa_code.as_ref(),
        })
    ).await;

    assert_eq!(response.status().as_u16(), 401);
}


#[tokio::test]
async fn should_return_200_if_correct_code() {
    // Make sure to assert the auth cookie gets set
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let random_password = "password123";

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": random_password,
    });    
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    let (login_attempt_id, two_fa_code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(&random_email).unwrap())
        .await
        .unwrap();

    let response = app.post_verify_2fa(
        &serde_json::json!({
            "email": random_email,
            "loginAttemptId": login_attempt_id.as_ref(),
            "2FACode": two_fa_code.as_ref(),
        })
    ).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}