use auth_service::{AppState, Application, BannedStoreType, UserStoreType, TwoFACodeStoreType, prod};

#[tokio::main]
async fn main() {
    let user_store = UserStoreType::default();
    let banned_store = BannedStoreType::default();
    let two_fa_store = TwoFACodeStoreType::default();

    let app_state = AppState::new(user_store, banned_store, two_fa_store);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
