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
    Clear,
}

impl Value {
    pub fn is_upvote(&self) -> bool {
        if let Value::Upvote = self {
            true
        } else {
            false
        }
    }

    pub fn is_downvote(&self) -> bool {
        if let Value::Downvote = self {
            true
        } else {
            false
        }
    }
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "upvote" => Ok(Value::Upvote),
            "downvote" => Ok(Value::Downvote),
            "clear" => Ok(Value::Clear),
            _ => Err(()),
        }
    }
}

impl Table for Vote {
    fn table_type(&self) -> crate::TableType {
        TableType::Votes
    }

    fn id(&self) -> Uuid {
        self.id
    }
}

impl ToTableType for Vote {
    fn to_table_type() -> TableType {
        TableType::Votes
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

impl<'r> FromRow<'r, PgRow> for Vote {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Vote {
            id: Uuid::from_str(row.try_get("id")?).unwrap(),
            parent_id: Uuid::from_str(row.try_get("parent_id")?).unwrap(),
            author_id: Uuid::from_str(row.try_get("author_id")?).unwrap(),
            value: Value::from_str(row.try_get("value")?).unwrap(),
            time: row.try_get("time")?,
        })
    }
}
