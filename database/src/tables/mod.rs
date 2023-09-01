use std::{default, fmt::Display, str::FromStr};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::PgRow,
    types::{
        chrono::{DateTime, Local, NaiveDateTime, Utc},
        uuid, Uuid,
    },
    Encode, FromRow, Pool, Postgres,
};

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
    fn to_table_type() -> TableType;
    fn id(&self) -> &UuidWrapper;
}

#[async_trait]
pub trait Insert {
    async fn insert(&self, pool: &Pool<Postgres>);
}

#[async_trait]
pub trait Select {
    async fn select_by_id(pool: &Pool<Postgres>, id: &String) -> Option<Self>
    where
        Self: Sized;

    async fn select_where(pool: &Pool<Postgres>, query: &str) -> Vec<Self>
    where
        Self: Sized;
}

#[async_trait]
pub trait Update {
    async fn update(&self, pool: &Pool<Postgres>);
}

#[async_trait]
pub trait Delete {
    async fn delete(&self, pool: &Pool<Postgres>);
}

#[derive(PartialEq, Debug, Clone)]
pub struct UuidWrapper(pub(crate) Uuid);

impl Serialize for UuidWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for UuidWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let uuid_str = String::deserialize(deserializer)?;
        let uuid = Uuid::parse_str(&uuid_str).map_err(serde::de::Error::custom)?;
        Ok(UuidWrapper(uuid))
    }
}

impl Display for UuidWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// impl From<String> for UuidWrapper {
//     fn from(value: String) -> Self {
//         println!("radu==={value}");
//         todo!()
//     }
// }

// impl From<()> for UuidWrapper {
//     fn from(value: ()) -> Self {
//         println!("radu===");
//         todo!()
//     }
// }
// impl FromStr for UuidWrapper {
//     type Err = uuid::Error;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(Self(Uuid::from_str(s)?))
//     }
// }

impl From<String> for UuidWrapper {
    fn from(value: String) -> Self {
        Self(Uuid::from_str(value.as_str()).unwrap())
    }
}

impl From<()> for UuidWrapper {
    fn from(_value: ()) -> Self {
        Self(Uuid::default())
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct DateTimeWrapper(pub(crate) NaiveDateTime);

impl From<NaiveDateTime> for DateTimeWrapper {
    fn from(value: NaiveDateTime) -> Self {
        let v = value.to_string();
        let vv = v.as_str();
        let d = NaiveDateTime::from_str(vv).unwrap();
        Self(d)
    }
}
