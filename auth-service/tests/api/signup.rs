use auth_service::api::dtos::{ErrorResponse, SignUpResponse};

use super::helpers::{TestApp, get_random_email};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    // TODO: add more malformed input test cases
    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "password": true
        }),
        serde_json::json!({
            "email": true,
            "password": random_email
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await; // call `post_signup`
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;

    let test_cases = [
        (get_random_email(), "password123", true),
        (get_random_email(), "password123", false),
    ];

    let expected_response = SignUpResponse {
        message: "User created successfully!".to_string(),
    };

    for test_case in test_cases.iter() {
        let (email, password, requires_2fa) = test_case;
        let request_body = serde_json::json!({
            "email": email,
            "password": password,
            "requires2FA": requires_2fa
        });

        let response = app.post_signup(&request_body).await; // call `post_signup`
        assert_eq!(response.status().as_u16(), 201);

        assert_eq!(
            response
                .json::<SignUpResponse>()
                .await
                .expect("Could not deserialize response body to UserBody"),
            expected_response
        );
    }
}

#[tokio::test]
async fn should_return_409_if_existing_user() {
    let app = TestApp::new().await;

    let email = get_random_email();
    let password = "password123";

    let request_body = serde_json::json!({
        "email": email,
        "password": password,
        "requires2FA": true
    });
    // First sign-up attempt should succeed
    let response = app.post_signup(&request_body).await; // call `post_signup`
    assert_eq!(response.status().as_u16(), 201);

    // Second sign-up attempt with the same email should fail with 409
    let response = app.post_signup(&request_body).await; // call `post_signup`
    assert_eq!(response.status().as_u16(), 409);
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let email = get_random_email();
    let password = "password123";

    let input = [
        serde_json::json!({
            "email": "",
            "password": password,
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "invalidemail.com",
            "password": password,
            "requires2FA": true
        }),
        serde_json::json!({
            "email": email,
            "password": "short",
            "requires2FA": true
        }),
    ];
    // The signup route should return a 400 HTTP status code if an invalid input is sent.
    // The input is considered invalid if:
    // - The email is empty or does not contain '@'
    // - The password is less than 8 characters

    // Create an array of invalid inputs. Then, iterate through the array and
    // make HTTP calls to the signup route. Assert a 400 HTTP status code is returned.
    for i in input.iter() {
        let response = app.post_signup(i.into()).await;
        assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", i);

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    // Call the signup route twice. The second request should fail with a 409 HTTP status code
    let app = TestApp::new().await;
    let email = get_random_email();
    let password = "password123";
    let i = serde_json::json!({
        "email": email,
        "password": password,
        "requires2FA": true
    });
    let _ = app.post_signup(&i).await;
    let response = app.post_signup(&i).await;
    assert_eq!(response.status().as_u16(), 409);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );
}
