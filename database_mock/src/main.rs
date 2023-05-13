mod database;
use std::error::Error;

use database::connector::DBConnection;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let a = DBConnection::new();
    Ok(())
}
