use async_trait::async_trait;
use sqlx::{postgres::PgRow, query_as, FromRow, Pool, Postgres, Row};

use crate::{DateTimeWrapper, Delete, Insert, Select, Table, TableType, Update, UuidWrapper};

pub struct Post {
    pub id: UuidWrapper,
    pub owner_id: UuidWrapper,
    pub create_time: DateTimeWrapper,
    // TODO: make u32
    pub likes: i32,
    pub dislikes: i32,
    pub text: String,
    pub image: Option<String>,
    pub video: Option<String>,
    pub audio: Option<String>,
}

impl Table for Post {
    fn table_type(&self) -> crate::TableType {
        TableType::Posts
    }

    fn to_table_type() -> TableType {
        TableType::Posts
    }

    fn id(&self) -> &UuidWrapper {
        &self.id
    }
}

#[async_trait]
impl Insert for Post {
    async fn insert(&self, pool: &Pool<Postgres>) {}
}

#[async_trait]
impl Select for Post {
    async fn select_by_id(pool: &Pool<Postgres>, id: &String) -> Option<Self>
    where
        Self: Sized,
    {
        query_as!(Self, "SELECT * FROM posts WHERE id=$1", &id.to_string())
            .fetch_optional(pool)
            .await
            .unwrap()
    }

    async fn select_where(pool: &Pool<Postgres>, query: &str) -> Vec<Self>
    where
        Self: Sized,
    {
        query_as::<_, Self>(&("SELECT * FROM posts ".to_owned() + query))
            .fetch_all(pool)
            .await
            .unwrap()
    }
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
            id: UuidWrapper::from(row.try_get("id")?),
            owner_id: UuidWrapper::from(row.try_get("owner_id")?),
            create_time: DateTimeWrapper::default(),
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
        let res = DBConnection::new()
            .await
            .select_where::<Post>(vec![(
                "owner_id",
                &"b0648b6c-f3a7-4789-bf57-3b44e15029d9".to_owned(),
            )])
            .await;
        assert!(!res.is_empty());
    }
}
