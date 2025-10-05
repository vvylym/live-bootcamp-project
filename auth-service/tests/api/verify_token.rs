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
        let response = app.post_verify_token(test_case).await; // call `post_signup`
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}
