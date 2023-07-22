use std::{
    collections::HashMap,
    env,
    fmt::Display,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::Path,
    str::FromStr,
    vec,
};

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    query,
    types::Uuid,
    FromRow, Pool, Postgres,
};

use crate::{Delete, Insert, Table, ToTableType, Update};

pub struct DBConnection {
    pool: Pool<Postgres>,
    content: HashMap<String, Vec<UuidWrapper>>,
}

impl DBConnection {
    pub async fn new() -> Self {
        dotenv().ok();
        if Path::new("created.json").exists() {
        } else {
            let mut file = File::create("created.json").unwrap();
            file.write_all(b"{}").unwrap();
        }
        let file = File::open("created.json").expect("could not find file");
        let lines = BufReader::new(file).lines();
        let content = lines.into_iter().flatten().collect::<String>();

        Self {
            pool: PgPoolOptions::new()
                .max_connections(3)
                .connect(&env::var("DATABASE_URL").unwrap())
                .await
                .unwrap(),
            content: serde_json::from_str(if content.is_empty() {
                "{}"
            } else {
                content.as_str()
            })
            .unwrap(),
        }
    }

    pub async fn insert<Tbl>(&mut self, table: &Tbl)
    where
        Tbl: Table + Insert,
    {
        table.insert(&self.pool).await;
        self.add(table.table_type(), &table.id())
    }

    pub async fn select_by_id<Tbl>(&self, id: &String) -> Option<Tbl>
    where
        Tbl: Table + Send + Unpin + for<'r> FromRow<'r, PgRow> + ToTableType,
    {
        // TODO: bind here
        let query = format!("SELECT * FROM {} WHERE id='{}'", Tbl::to_table_type(), id);

        sqlx::query_as::<_, Tbl>(query.as_str())
            .fetch_optional(&self.pool)
            .await
            .unwrap()
    }

    pub async fn select_where<Tbl>(&self, pairs: Vec<(&'static str, &String)>) -> Vec<Tbl>
    where
        Tbl: Table + Send + Unpin + for<'r> FromRow<'r, PgRow> + ToTableType,
    {
        let mut query = format!("SELECT * FROM {} WHERE", Tbl::to_table_type());
        for (key, val) in pairs {
            query.push_str(format!(" {key}='{val}' AND").as_str());
        }
        let query = query.trim_end_matches(" AND");

        sqlx::query_as::<_, Tbl>(query)
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    pub async fn update<Tbl>(&self, table: &Tbl)
    where
        Tbl: Table + Update,
    {
        table.update(&self.pool).await;
    }

    pub async fn delete<Tbl>(&mut self, table: &Tbl)
    where
        Tbl: Table + Delete,
    {
        table.delete(&self.pool).await;
        self.delete_cached(table.table_type(), &table.id());
    }

    pub async fn delete_by_id(&mut self, table: TableType, id: &String) {
        query(format!("DELETE FROM {} WHERE id='{}'", table, id).as_str())
            .execute(&self.pool)
            .await
            .unwrap();
        self.delete_cached(table, &Uuid::from_str(id.as_str()).unwrap());
    }

    fn delete_cached(&mut self, table: TableType, id: &Uuid) {
        if let Some(v) = self.content.get_mut(&table.to_string()) {
            v.retain(|e| e.0 == *id);
            if v.is_empty() {
                self.content.remove(&table.to_string());
            }
        }
    }

    fn add(&mut self, table: TableType, uuid: &Uuid) {
        if let Some(v) = self.content.get_mut(&table.to_string()) {
            v.push(UuidWrapper(*uuid));
        } else {
            self.content
                .insert(table.to_string(), vec![UuidWrapper(*uuid)]);
        }
    }
}

impl Drop for DBConnection {
    fn drop(&mut self) {
        let text = serde_json::to_string_pretty(&self.content).expect("serde serialization failed");
        fs::write("created.json", text).expect("failed writing to file");
    }
}

pub struct UuidWrapper(Uuid);

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

pub enum TableType {
    Comments,
    Posts,
    Users,
    Votes,
}

impl Display for TableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                TableType::Comments => "comments",
                TableType::Posts => "posts",
                TableType::Users => "users",
                TableType::Votes => "votes",
            }
        )
    }
}
