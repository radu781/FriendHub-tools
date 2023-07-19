use async_trait::async_trait;
use fake::faker::address::en::{CityName, CountryName};
use fake::faker::chrono::en::DateTime;
use fake::faker::internet::raw::SafeEmail;
use fake::faker::name::en::FirstName;
use fake::faker::name::en::LastName;
use fake::locales::EN;
use fake::uuid::UUIDv4;
use fake::{Dummy, Fake, Faker};
use rand::rngs::StdRng;
use rand::Rng;
use sqlx::types::chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use sqlx::types::Uuid;
use sqlx::{query, Pool, Postgres};

// use openssl::sha::sha256;

use crate::connector::TableType;
pub struct Comments {}

pub struct ImGroupMembers {}

pub struct ImGroups {}

pub struct Message {}

pub struct PGroupMembers {}

pub struct Pages {}

pub struct PagesGroups {}

pub struct Posts {}

pub struct Relationships {}

pub struct Replies {}

pub struct Scripts {}

pub struct Tokens {}

#[derive(Debug, Dummy)]
pub struct User {
    pub id: Uuid,
    #[dummy(faker = "FirstName()")]
    pub first_name: String,
    #[dummy(faker = "FirstName()")]
    middle_name: String,
    #[dummy(faker = "LastName()")]
    last_name: String,
    join_time: NaiveDateTime,
    #[dummy(faker = "CountryName()")]
    country: String,
    #[dummy(faker = "CityName()")]
    city: String,
    education: String,
    extra: String,
    profile_picture: String,
    banner_picture: String,
    #[dummy(faker = "SafeEmail(EN)")]
    email: String,
    password: String,
    permissions: String,
}

pub struct Votes {}

pub trait Adjust {
    fn adjust(self) -> Self;
}

pub trait Table {
    fn table_type(&self) -> TableType;
    fn id(&self) -> Uuid;
}

#[async_trait]
pub trait Insert {
    async fn insert(&self, pool: &Pool<Postgres>);
}

#[async_trait]
pub trait Delete {
    async fn delete(&self, pool: &Pool<Postgres>);
}

impl Table for User {
    fn table_type(&self) -> TableType {
        TableType::Users
    }

    fn id(&self) -> Uuid {
        self.id
    }
}

impl Adjust for User {
    fn adjust(mut self) -> Self {
        let uuid = Uuid::new_v4();
        self.id = uuid;

        let mut rng = rand::thread_rng();
        let educations = vec!["highschool", "college"];
        let i = rng.gen_range(0..educations.len());
        self.education = educations[i].to_owned();

        let permissions = vec!["admin", "tester", "demo"];
        let i = rng.gen_range(0..permissions.len());
        self.permissions = permissions[i].to_owned();

        // let password = sha256(self.password.as_bytes()).to_string();
        // self.password = password;
        let password: [u8; 32] = rng.gen();
        self.password = password.iter().map(|byte| format!("{:02x}", byte)).collect();

        let date = NaiveDate::from_ymd_opt(
            rng.gen_range(2020..=2023),
            rng.gen_range(1..=12),
            rng.gen_range(1..=28),
        )
        .expect("failed to create date");
        let time = NaiveTime::from_hms_opt(
            rng.gen_range(0..24),
            rng.gen_range(0..60),
            rng.gen_range(0..60),
        )
        .expect("failed to create time");
        self.join_time = NaiveDateTime::new(date, time);

        self
    }
}

#[async_trait]
impl Insert for User {
    async fn insert(&self, pool: &Pool<Postgres>) {
        query!(
            r#"INSERT INTO users VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)"#,
            &self.id.to_string(), self.first_name, self.middle_name, self.last_name,
            self.join_time, self.country, self.city, self.education,
            self.extra, self.profile_picture, self.banner_picture, self.email,
            self.password, self.permissions
        )
        .execute(pool)
        .await
        .unwrap();
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
