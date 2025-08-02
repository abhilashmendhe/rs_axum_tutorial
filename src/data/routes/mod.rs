use axum::{routing::{get, post}, Extension, Router};
use sea_orm::{DatabaseConnection};

use crate::routes::{create_task::create_task, custom_json_extractor::custom_json_extractor, validate_with_serde::validate_with_serde};

pub mod validate_with_serde;
pub mod custom_json_extractor;
pub mod create_task;

pub async fn create_routes(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/hello-world", get(async || { "Hello, World" }))
        .route("/validate_data", post(validate_with_serde))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/tasks", post(create_task))
        .layer(Extension(database))
}