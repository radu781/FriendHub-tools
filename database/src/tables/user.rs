use async_trait::async_trait;
use sqlx::postgres::PgRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{query, query_as, FromRow, Pool, Postgres, Row};

use crate::{
    Adjust, Delete, Insert, Select, Table, TableType, Update, UuidWrapper,
};

#[derive(Debug)]
pub struct User {
    id: UuidWrapper,
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    join_time: NaiveDateTime,
    country: Option<String>,
    city: Option<String>,
    education: Option<String>,
    extra: Option<String>,
    profile_picture: Option<String>,
    banner_picture: Option<String>,
    email: String,
    password: String,
    permissions: Option<String>,
}

impl Table for User {
    fn table_type(&self) -> TableType {
        TableType::Users
    }

    fn to_table_type() -> TableType {
        TableType::Users
    }

    fn id(&self) -> &UuidWrapper {
        &self.id
    }
}

impl Adjust for User {
    fn adjust(mut self) -> Self {
        // let uuid = Uuid::new_v4();
        // self.id = uuid;

        // let mut rng = rand::thread_rng();
        // let educations = vec!["highschool", "college"];
        // let i = rng.gen_range(0..educations.len());
        // self.education = Some(educations[i].to_owned());

        // let permissions = vec!["admin", "tester", "demo"];
        // let i = rng.gen_range(0..permissions.len());
        // self.permissions = permissions[i].to_owned();

        // // let password = sha256(self.password.as_bytes()).to_string();
        // // self.password = password;
        // let password: [u8; 32] = rng.gen();
        // self.password = password
        //     .iter()
        //     .map(|byte| format!("{:02x}", byte))
        //     .collect();

        self
    }
}

#[async_trait]
impl Insert for User {
    async fn insert(&self, pool: &Pool<Postgres>) {
        query!(
            "INSERT INTO users VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",
            &self.id.to_string(),
            self.first_name,
            self.middle_name,
            self.last_name,
            self.join_time,
            self.country,
            self.city,
            self.education,
            self.extra,
            self.profile_picture,
            self.banner_picture,
            self.email,
            self.password,
            self.permissions
        )
        .execute(pool)
        .await
        .unwrap();
    }
}

#[async_trait]
impl Select for User {
    async fn select_by_id(pool: &Pool<Postgres>, id: &String) -> Option<Self>
    where
        Self: Sized,
    {
        query_as!(Self, "SELECT * FROM users WHERE id=$1", &id.to_string())
            .fetch_optional(pool)
            .await
            .unwrap()
    }

    async fn select_where(pool: &Pool<Postgres>, query: &str) -> Vec<Self>
    where
        Self: Sized,
    {
        query_as::<_, Self>(&("SELECT * FROM users ".to_owned() + query))
            .fetch_all(pool)
            .await
            .unwrap()
    }
}

#[async_trait]
impl Update for User {
    async fn update(&self, pool: &Pool<Postgres>) {
        query!(
            "UPDATE users SET first_name=$1 WHERE id=$2",
            &self.first_name,
            &self.id.to_string()
        )
        .execute(pool)
        .await
        .ok();
    }
}

#[async_trait]
impl Delete for User {
    async fn delete(&self, pool: &Pool<Postgres>) {
        query!("DELETE FROM users WHERE id=$1", &self.id.to_string())
            .execute(pool)
            .await
            .unwrap();
    }
}

impl<'r> FromRow<'r, PgRow> for User {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(User {
            id: UuidWrapper::from(row.try_get("id")?),
            first_name: row.try_get("first_name")?,
            middle_name: row.try_get("middle_name")?,
            last_name: row.try_get("last_name")?,
            join_time: NaiveDateTime::default(),
            country: row.try_get("country")?,
            city: row.try_get("city")?,
            education: row.try_get("education")?,
            extra: row.try_get("extra")?,
            profile_picture: row.try_get("profile_picture")?,
            banner_picture: row.try_get("banner_picture")?,
            email: row.try_get("email")?,
            password: row.try_get("password")?,
            permissions: row.try_get("permissions")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Adjust, DBConnection, Table, User};
    use fake::{Fake, Faker};

    // #[test]
    // fn create_instance() {
    //     // let user = Faker.fake::<User>().adjust();
    //     // assert_ne!(user.first_name, "".to_owned());
    //     // assert_ne!(user.middle_name, "".to_owned());
    //     // assert_ne!(user.last_name, "".to_owned());
    // }

    // #[tokio::test]
    // async fn insert_in_db() {
    //     let (mut db, user) = setup().await;

    //     let res = db.select_by_id::<User>(&user.id.to_string()).await;
    //     assert_eq!(res.unwrap().id, user.id);

    //     teardown(db, user).await;
    // }

    // #[tokio::test]
    // async fn delete_in_db_id() {
    //     let (mut db, user) = setup().await;

    //     db.delete_by_id(user.table_type(), &user.id.to_string()).await;
    //     let res = db.select_by_id::<User>(&user.id.to_string()).await;
    //     assert!(res.is_none());
    // }

    // #[tokio::test]
    // async fn delete_in_db_whole() {
    //     let (mut db, user) = setup().await;

    //     db.delete(&user).await;
    //     let res = db.select_by_id::<User>(&user.id.to_string()).await;
    //     assert!(res.is_none());
    // }

    // #[tokio::test]
    // async fn update_in_db() {
    //     let (mut db, mut user) = setup().await;

    //     user.first_name = "EDITED".to_string();
    //     db.update(&user).await;
    //     let table_user = db.select_by_id::<User>(&user.id.to_string()).await.unwrap();
    //     assert_eq!(table_user.first_name, "EDITED".to_string());

    //     teardown(db, user).await;
    // }

    // async fn setup() -> (DBConnection, User) {
    //     let user = Faker.fake::<User>().adjust();
    //     let mut db = DBConnection::new().await;
    //     db.insert(&user).await;
    //     (db, user)
    // }

    // async fn teardown(mut db: DBConnection, user: User) {
    //     db.delete(&user).await;
    // }
}
