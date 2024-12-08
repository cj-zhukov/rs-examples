use std::{env as std_env, sync::LazyLock};

use dotenvy::dotenv;

pub mod env {
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
}

pub const PG_TABLE_NAME: &str = "users";
pub const PG_MAX_CONNECTIONS: u32 = 10;

pub static DATABASE_URL: LazyLock<String> = LazyLock::new(|| {
    dotenv().ok();
    let secret = std_env::var(env::DATABASE_URL_ENV_VAR)
        .expect("DATABASE_URL_ENV_VAR must be set.");
    if secret.is_empty() {
        panic!("DATABASE_URL_ENV_VAR must not be empty.");
    }
    secret
});