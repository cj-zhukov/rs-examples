use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::domain::UserStore;
use crate::error::AppError;

pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Users {
    pub id: String,
    pub name: String,
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    async fn get_user(&self, id: &str) -> Result<String, AppError> {
        let sql = format!("select * from users where id = $1");
        let query = sqlx::query_as::<_, Users>(&sql);
        query
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(AppError::SqlxError)?
            .map(|u| {
                Ok(u.name)
            })
            .ok_or(AppError::UserNotFound)?
    }
}