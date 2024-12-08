use crate::domain::UserStore;
use crate::error::AppError;

// App is using generics with trait bounds
pub struct App<T: UserStore> {
    pub user_store: T,
}

impl<T: UserStore> App<T> {
    pub fn new(user_store: T) -> Self {
        Self { user_store }
    }

    pub async fn fetch_user_name(&self, id: &str) -> Result<String, AppError> {
        self.user_store.get_user(id).await
    }
}