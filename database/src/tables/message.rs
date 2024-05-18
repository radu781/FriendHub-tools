use async_trait::async_trait;
use serde::Deserialize;
use sqlx::{query, Pool, Postgres};

use crate::{Insert, Table, UuidWrapper};

#[derive(Deserialize, Debug, Clone)]
pub struct Message {
    #[serde(skip)]
    db_id: UuidWrapper,
    pub from: UuidWrapper,
    pub to_user: Option<UuidWrapper>,
    pub to_group: Option<Vec<UuidWrapper>>,
    pub text: String,
}

impl Message {
    fn to_user_string(&self) -> String {
        self.to_user
            .clone()
            .map_or("NULL".to_owned(), |x| x.to_string())
    }

    fn to_group_string(&self) -> String {
        self.to_group
            .clone()
            .map_or("NULL".to_owned(), |x| Self::group_to_string(x))
    }

    pub fn group_to_string(mut value: Vec<UuidWrapper>) -> String {
        value.sort();
        let uuid_len = value[0].to_string().len();
        let mut room_name = String::with_capacity(uuid_len);
        for i in 0..uuid_len {
            for user in &mut *value {
                room_name += user.to_string().as_bytes()[i].to_string().as_str();
            }
        }
        room_name
    }
}

impl Table for Message {
    fn table_type(&self) -> crate::TableType {
        crate::TableType::Messages
    }

    fn to_table_type() -> crate::TableType {
        crate::TableType::Messages
    }

    fn id(&self) -> &UuidWrapper {
        &self.db_id
    }
}

#[async_trait]
impl Insert for Message {
    async fn insert(&self, pool: &Pool<Postgres>) {
        query!(
            "INSERT INTO messages VALUES($1, $2, $3, $4, $5)",
            self.db_id.to_string(),
            self.from.to_string(),
            self.to_user_string(),
            self.to_group_string(),
            self.text,
        )
        .execute(pool)
        .await
        .unwrap();
    }
}
