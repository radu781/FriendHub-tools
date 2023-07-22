use std::str::FromStr;

use async_trait::async_trait;
use sqlx::{
    postgres::PgRow,
    types::{
        chrono::{DateTime, Local},
        Uuid,
    },
    FromRow, Pool, Postgres, Row,
};

use crate::{Delete, Insert, Select, Table, TableType, ToTableType, Update};

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

impl ToTableType for Post {
    fn to_table_type() -> TableType {
        TableType::Posts
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

impl<'r> FromRow<'r, PgRow> for Post {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Post {
            id: Uuid::from_str(row.try_get("id")?).unwrap(),
            owner_id: Uuid::from_str(row.try_get("owner_id")?).unwrap(),
            create_time: DateTime::default(),
            likes: 1,
            dislikes: 1,
            text: row.try_get("text")?,
            image: row.try_get("image")?,
            video: row.try_get("video")?,
            audio: row.try_get("audio")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{DBConnection, Post};

    #[tokio::test]
    async fn select_where() {
        let mut db = DBConnection::new().await;
        let res = db
            .select_where::<Post>(vec![(
                "owner_id",
                &"b0648b6c-f3a7-4789-bf57-3b44e15029d9".to_owned(),
            )])
            .await;
        assert!(!res.is_empty());
    }
}
