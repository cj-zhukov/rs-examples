use std::collections::HashMap;
use std::sync::Arc;

use color_eyre::Result;
use tokio::sync::RwLock;

use crate::domain::UserStore;
use crate::error::AppError;

pub struct HashMapUserStore {
    pub user_store: Arc<RwLock<HashMap<String, String>>>,
} 

impl HashMapUserStore {
    pub fn new() -> Self {
        Self { user_store: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub fn default() -> Self {
        let mut map = HashMap::new();

        map.insert("a".to_string(), "foo".to_string());
        map.insert("b".to_string(), "bar".to_string());
        map.insert("c".to_string(), "baz".to_string());

        Self { user_store: Arc::new(RwLock::new(map)) }
    }
}

#[async_trait::async_trait]
impl UserStore for HashMapUserStore {
    async fn get_user(&self, id: &str) -> Result<String, AppError> {
        let user_store = self.user_store.read().await;
        let res = user_store.get(id).ok_or(AppError::UserNotFound)?;

        Ok(res.to_string())
    }
}