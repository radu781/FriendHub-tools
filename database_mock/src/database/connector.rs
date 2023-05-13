use std::env;

use dotenv::dotenv;
use lazy_static::lazy_static;
use sqlx::{
    postgres::PgPoolOptions,
    Pool, Postgres,
};
use tokio::runtime::Runtime;

pub struct DBConnection {
    pub pool: Option<Pool<Postgres>>,
}

lazy_static! {
    pub static ref DB_CONNECTION: DBConnection = Runtime::new()
        .unwrap()
        .block_on(async { DBConnection::new().await });
}

impl DBConnection {
    pub async fn new() -> Self {
        dotenv().ok();
        Self {
            pool: Some(
                PgPoolOptions::new()
                    .max_connections(3)
                    .connect(&env::var("DATABASE_URL").unwrap())
                    .await
                    .unwrap(),
            ),
        }
    }
}
