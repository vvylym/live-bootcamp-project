use super::helpers::TestApp;

#[tokio::test]
async fn test_signup() {
    let app = TestApp::new().await;

    let response = app.signup().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type"), None);
}
