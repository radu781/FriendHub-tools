mod database;
use std::error::Error;

use database::{connector::DBConnection, tables::Adjust};
use fake::{Fake, Faker};

use crate::database::tables::User;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut db_connection = DBConnection::new().await;
    let user = Faker.fake::<User>().adjust();
    db_connection.insert(&user).await;
    Ok(())
}
