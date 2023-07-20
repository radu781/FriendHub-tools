use async_trait::async_trait;
use sqlx::{types::Uuid, Pool, Postgres};

use crate::{Delete, Insert, Select, Table, TableType, Update};

pub struct Comment {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub body: String,
    pub likes: u32,
    pub dislikes: u32,
}

impl Table for Comment {
    fn table_type(&self) -> crate::TableType {
        TableType::Comments
    }

    fn id(&self) -> Uuid {
        self.id
    }
}

#[async_trait]
impl Insert for Comment {
    async fn insert(&self, pool: &Pool<Postgres>) {}
}

#[async_trait]
impl Select for Comment {
    async fn select(&self, pool: &Pool<Postgres>) {}
}

#[async_trait]
impl Update for Comment {
    async fn update(&self, pool: &Pool<Postgres>) {}
}

#[async_trait]
impl Delete for Comment {
    async fn delete(&self, pool: &Pool<Postgres>) {}
}
