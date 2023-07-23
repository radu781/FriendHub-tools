use async_trait::async_trait;
use sqlx::{postgres::PgRow, query_as, FromRow, Pool, Postgres, Row};

use crate::{Delete, Insert, Select, Table, TableType, Update, UuidWrapper};

pub struct Comment {
    pub id: UuidWrapper,
    pub parent_id: UuidWrapper,
    pub body: String,
    // TODO: use u32
    pub likes: i32,
    pub dislikes: i32,
}

impl Table for Comment {
    fn table_type(&self) -> crate::TableType {
        TableType::Comments
    }

    fn to_table_type() -> TableType {
        TableType::Comments
    }

    fn id(&self) -> &UuidWrapper {
        &self.id
    }
}

#[async_trait]
impl Insert for Comment {
    async fn insert(&self, pool: &Pool<Postgres>) {}
}

#[async_trait]
impl Select for Comment {
    async fn select_by_id(pool: &Pool<Postgres>, id: &String) -> Option<Self>
    where
        Self: Sized,
    {
        query_as!(Self, "SELECT * FROM comments WHERE id=$1", &id.to_string())
            .fetch_optional(pool)
            .await
            .unwrap()
    }

    async fn select_where(pool: &Pool<Postgres>, query: &str) -> Vec<Self>
    where
        Self: Sized,
    {
        vec![]
    }
}

#[async_trait]
impl Update for Comment {
    async fn update(&self, pool: &Pool<Postgres>) {}
}

#[async_trait]
impl Delete for Comment {
    async fn delete(&self, pool: &Pool<Postgres>) {}
}

impl<'r> FromRow<'r, PgRow> for Comment {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Comment {
            id: UuidWrapper::from(row.try_get("id")?),
            parent_id: UuidWrapper::from(row.try_get("parent_id")?),
            body: row.try_get("body")?,
            likes: row.try_get("likes")?,
            dislikes: row.try_get("dislikes")?,
        })
    }
}
