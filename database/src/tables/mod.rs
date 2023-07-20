use async_trait::async_trait;
use sqlx::{types::Uuid, Pool, Postgres};

use crate::TableType;

pub mod comment;
pub mod post;
pub mod user;
pub mod vote;

pub use comment::*;
pub use post::*;
pub use user::*;
pub use vote::*;

pub trait Adjust {
    fn adjust(self) -> Self;
}

pub trait Table {
    fn table_type(&self) -> TableType;
    fn id(&self) -> Uuid;
}

#[async_trait]
pub trait Insert {
    async fn insert(&self, pool: &Pool<Postgres>);
}

#[async_trait]
pub trait Select {
    async fn select(&self, pool: &Pool<Postgres>);
}

#[async_trait]
pub trait Update {
    async fn update(&self, pool: &Pool<Postgres>);
}

#[async_trait]
pub trait Delete {
    async fn delete(&self, pool: &Pool<Postgres>);
}
