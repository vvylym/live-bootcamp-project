use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{AppState, Application, prod};

use auth_service::services::*;

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let banned_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let two_fa_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
    let email_client = Arc::new(RwLock::new(MockEmailClient{}));

    let app_state = AppState::new(user_store, banned_store, two_fa_store, email_client);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
