/*
clear; sea-orm-cli  generate entity -o ./database -u postgres://postgres:keyoarbcat@localhost:5433/postgres
*/

use sea_orm::Database;
use tokio::net::TcpListener;

use crate::routes::create_routes;

pub async fn run(database_uri: &str) {
    let database = Database::connect(database_uri).await.unwrap();

    let app = create_routes(database).await;

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
