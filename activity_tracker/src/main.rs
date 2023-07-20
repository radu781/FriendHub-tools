use std::{env, process::exit};

use crate::compute::*;
mod compute;

const UUID_NOT_FOUND: i32 = 1;
const NO_ARGS: i32 = 2;

#[tokio::main]
async fn main() {
    match env::args().skip(1).collect::<Vec<_>>().first() {
        Some(val) => {
            let res = compute(val).await;
            match res {
                Some(score) => println!("{score}"),
                None => exit(UUID_NOT_FOUND),
            }
        }
        None => exit(NO_ARGS),
    }
}
