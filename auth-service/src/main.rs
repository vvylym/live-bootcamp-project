use auth_service::{AppState, Application, UserStoreType, prod};

#[tokio::main]
async fn main() {
    let user_store = UserStoreType::default();
    let app_state = AppState::new(user_store);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
