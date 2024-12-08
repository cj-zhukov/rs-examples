use color_eyre::Result;

use crate::error::AppError;

#[async_trait::async_trait]
pub trait UserStore {
    async fn get_user(&self, id: &str) -> Result<String, AppError>;
}