pub mod helper;
pub mod database;
pub mod routes;

use std::env::current_dir;

use crate::helper::run;

#[tokio::main]
async fn main() {

    let env_path = format!("{}/src/data/.env",current_dir().unwrap().display());
    dotenv::from_path(env_path).ok();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    println!("DB url: {}", db_url);
    run(&db_url).await;
}