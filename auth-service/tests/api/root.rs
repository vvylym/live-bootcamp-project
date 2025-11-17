use super::helpers::TestApp;

#[tokio::test]
async fn test_root() {
    let mut app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert!(
        response
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("text/html")
    );

    let _ = app.clean_up().await;
}
