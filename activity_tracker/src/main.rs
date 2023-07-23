use std::{env, process::exit};

use crate::compute::*;
mod compute;

const USAGE: &str = r#"Usage: activity_tracker <user_id> <date>
- user_id is a UUID (b0648b6c-f3a7-4789-bf57-3b44e15029d9)
- date uses the format YYYY-MM-DD (2023-05-14)"#;

#[tokio::main]
async fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.len() {
        1 => {
            println!("{USAGE}");
            exit(ComputeError::NoArgs.into());
        }
        2 => {
            let res = compute(&args[0], &args[1]).await;
            match res {
                Ok(score) => println!("{score}"),
                Err(e) => exit(e.into()),
            }
        }
        _ => {
            println!("{USAGE}");
            exit(ComputeError::NoArgs.into());
        }
    };
}
