use axum::{routing::{delete, get, patch, post, put}, Extension, Router};
use sea_orm::{DatabaseConnection};

use crate::routes::{create_task::create_task, custom_json_extractor::custom_json_extractor, delete_task::delete_task, get_tasks::{get_all_tasks, get_one_task}, partial_update_tasks::partial_update, update_tasks::atomic_update, validate_with_serde::validate_with_serde};

pub mod validate_with_serde;
pub mod custom_json_extractor;
pub mod create_task;
pub mod get_tasks;
pub mod update_tasks;
pub mod partial_update_tasks;
pub mod delete_task;

pub async fn create_routes(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/hello-world", get(async || { "Hello, World" }))
        .route("/validate_data", post(validate_with_serde))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/tasks", post(create_task).get(get_all_tasks))
        .route("/tasks/{:task_id}", get(get_one_task))
        .route("/tasks/{:task_id}", put(atomic_update))
        .route("/tasks/{:task_id}", patch(partial_update))
        .route("/tasks/{:task_id}", delete(delete_task))
        .layer(Extension(database))
}