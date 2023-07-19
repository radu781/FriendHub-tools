use std::env;
use fake::{Dummy, Fake, Faker};

use crate::compute::*;
use database::{DBConnection, TableType, User, Adjust};
use tokio::main;
mod compute;

#[tokio::main]
async fn main() {
    let b = env::var("DATABASE_URL").unwrap();
    let a = DBConnection::new().await;
    Faker.fake::<User>().adjust();
}
