use super::helpers::TestApp;

#[tokio::test]
async fn test_verify_token() {
    let app = TestApp::new().await;

    let response = app.verify_token().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type"), None);
}
