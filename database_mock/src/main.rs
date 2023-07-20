use database::{Adjust, DBConnection, User};
use fake::{Fake, Faker};

#[tokio::main]
async fn main() {
    let mut db_connection = DBConnection::new().await;
    let user = Faker.fake::<User>().adjust();
    db_connection.insert(&user).await;
}
