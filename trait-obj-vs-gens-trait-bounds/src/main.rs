use std::sync::Arc;

use trait_obj_vs_generics_trait_bounds as myapp;
use myapp::app::app_gen_trait_bounds::App as App1;
use myapp::app::app_trait_obj::App as App2;
use myapp::service::{HashMapUserStore, PostgresUserStore};
use myapp::utils::{configure_postgresql, DATABASE_URL};

use color_eyre::Result;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().expect("Failed to install color_eyre");

    example_trait_objects().await?;
    example_generics_with_trait_bounds().await?;

    Ok(())
}

async fn example_generics_with_trait_bounds() -> Result<()> {
    // hashmap
    let user_store = HashMapUserStore::default();
    let app_state = App1::new(user_store);
    let res = app_state.fetch_user_name("a").await?;
    println!("{}", res);

    // postgres db
    let pg_pool = configure_postgresql(&DATABASE_URL).await;
    let user_store = PostgresUserStore::new(pg_pool);
    let app_state = App1::new(user_store);
    let res = app_state.fetch_user_name("a").await?;
    println!("{}", res);

    Ok(())
}

async fn example_trait_objects() -> Result<()> {
    // hashmap
    let user_store = HashMapUserStore::default();
    let user_store = Arc::new(RwLock::new(user_store));
    let app_state = App2::new(user_store);
    let res = app_state.fetch_user_name("a").await?;
    println!("{}", res);

    // postgres db
    let pg_pool = configure_postgresql(&DATABASE_URL).await;
    let user_store = PostgresUserStore::new(pg_pool);
    let user_store = Arc::new(RwLock::new(user_store));
    let app_state = App2::new(user_store);
    let res = app_state.fetch_user_name("a").await?;
    println!("{}", res);

    Ok(())
}
