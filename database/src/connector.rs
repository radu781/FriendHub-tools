use std::{
    collections::HashMap,
    env,
    fmt::Display,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::Path,
    vec,
};

use dotenv::dotenv;
use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    query, FromRow, Pool, Postgres,
};

use crate::{Delete, Insert, Select, Table, Update, UuidWrapper};

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
        self.add(table.table_type(), table.id())
    }

    pub async fn select_by_id<Tbl>(&self, id: &String) -> Option<Tbl>
    where
        Tbl: Table + Send + Unpin + for<'r> FromRow<'r, PgRow> + Select,
    {
        Tbl::select_by_id(&self.pool, id).await
    }

    pub async fn select_where<Tbl>(&self, pairs: Vec<(&'static str, &String)>) -> Vec<Tbl>
    where
        Tbl: Table + Send + Unpin + for<'r> FromRow<'r, PgRow> + Select,
    {
        let mut filter = "WHERE".to_owned();
        for (key, val) in pairs {
            filter.push_str(format!(" {key}='{val}' AND").as_str());
        }
        let filter = filter.trim_end_matches(" AND");

        Tbl::select_where(&self.pool, filter).await
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
        self.delete_cached(table.table_type(), table.id());
    }

    pub async fn delete_by_id(&mut self, table: TableType, id: &String) {
        query(format!("DELETE FROM {} WHERE id='{}'", table, id).as_str())
            .execute(&self.pool)
            .await
            .unwrap();
        self.delete_cached(table, &UuidWrapper::from(id.clone()))
    }

    fn delete_cached(&mut self, table: TableType, id: &UuidWrapper) {
        if let Some(v) = self.content.get_mut(&table.to_string()) {
            v.retain(|e| e == id);
            if v.is_empty() {
                self.content.remove(&table.to_string());
            }
        }
    }

    fn add(&mut self, table: TableType, uuid: &UuidWrapper) {
        if let Some(v) = self.content.get_mut(&table.to_string()) {
            v.push(uuid.clone());
        } else {
            self.content.insert(table.to_string(), vec![uuid.clone()]);
        }
    }
}

impl Drop for DBConnection {
    fn drop(&mut self) {
        let text = serde_json::to_string_pretty(&self.content).expect("serde serialization failed");
        fs::write("created.json", text).expect("failed writing to file");
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
