use std::sync::Arc;

use tokio::sync::RwLock;

use crate::domain::UserStore;
use crate::error::AppError;

pub type UserStoreType = Arc<RwLock<dyn UserStore + Send + Sync>>;

// App is using trait objects
#[derive(Clone)]
pub struct App {
    pub user_store: UserStoreType,
}

impl App {
    pub fn new(user_store: UserStoreType) -> Self {
        Self { user_store }
    }

    pub async fn fetch_user_name(&self, id: &str) -> Result<String, AppError> {
        let user_store = self.user_store.read().await;
        user_store.get_user(id).await
    }
}