use async_trait::async_trait;
use sqlx::{
    types::{
        chrono::{DateTime, Local},
        Uuid,
    },
    Pool, Postgres,
};

use crate::{Delete, Insert, Select, Table, TableType, Update};

pub struct Post {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub create_time: DateTime<Local>,
    pub likes: u32,
    pub dislikes: u32,
    pub text: String,
    pub image: Option<String>,
    pub video: Option<String>,
    pub audio: Option<String>,
}

impl Table for Post {
    fn table_type(&self) -> crate::TableType {
        TableType::Posts
    }

    fn id(&self) -> Uuid {
        self.id
    }
}

#[async_trait]
impl Insert for Post {
    async fn insert(&self, pool: &Pool<Postgres>) {}
}

#[async_trait]
impl Select for Post {
    async fn select(&self, pool: &Pool<Postgres>) {}
}

#[async_trait]
impl Update for Post {
    async fn update(&self, pool: &Pool<Postgres>) {}
}

#[async_trait]
impl Delete for Post {
    async fn delete(&self, pool: &Pool<Postgres>) {}
}
