use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::Path,
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

use crate::{Delete, Insert, Table, User};

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
            content: serde_json::from_str(&content).unwrap(),
        }
    }

    pub async fn insert(&mut self, table: &(impl Insert + Table)) {
        table.insert(&self.pool).await;
        self.add(table.table_type(), &table.id())
    }

    pub async fn select(&mut self, table: &(impl Insert + Table)) {}

    pub async fn select_id<'r, Tbl>(&mut self, table: TableType, id: &String) -> Option<User>
    where
        Tbl: Table + Send + Unpin + FromRow<'r, PgRow>,
    {
        // TODO: bind here
        let query = format!("SELECT * FROM {} WHERE id='{}'", table.to_string(), id);

        sqlx::query_as::<_, User>(query.as_str())
            // .bind(id)
            .fetch_optional(&self.pool)
            .await
            .unwrap()
    }

    pub async fn delete(&mut self, table: &(impl Delete + Table)) {
        table.delete(&self.pool).await;
        self.delete_cached(table.table_type(), &table.id());
    }

    pub async fn delete_id(&mut self, table: TableType, id: &Uuid) {
        query(format!("DELETE FROM {} WHERE id='{}'", table.to_string(), id).as_str())
            .execute(&self.pool)
            .await
            .unwrap();
        self.delete_cached(table, id);
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
    Users,
    Comments,
    Votes,
    Posts,
}

impl ToString for TableType {
    fn to_string(&self) -> String {
        match self {
            TableType::Users => "users".to_owned(),
            TableType::Comments => "comments".to_owned(),
            TableType::Votes => "votes".to_owned(),
            TableType::Posts => "posts".to_owned(),
        }
    }
}
