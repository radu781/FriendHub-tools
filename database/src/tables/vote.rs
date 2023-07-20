use async_trait::async_trait;
use sqlx::{types::{Uuid, chrono::{DateTime, Local}}, Pool, Postgres};

use crate::{Delete, Insert, Select, Table, TableType, Update};

pub struct Vote {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub author_id: Uuid,
    pub value: Value,
    pub time: DateTime<Local>,
}

pub enum Value {
    Upvote,
    Downvote,
    Clear
}

impl Table for Vote {
    fn table_type(&self) -> crate::TableType {
        TableType::Votes
    }

    fn id(&self) -> Uuid {
        self.id
    }
}

#[async_trait]
impl Insert for Vote {
    async fn insert(&self, pool: &Pool<Postgres>) {}
}

#[async_trait]
impl Select for Vote {
    async fn select(&self, pool: &Pool<Postgres>) {}
}

#[async_trait]
impl Update for Vote {
    async fn update(&self, pool: &Pool<Postgres>) {}
}

#[async_trait]
impl Delete for Vote {
    async fn delete(&self, pool: &Pool<Postgres>) {}
}
