mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;
mod query_params;
mod mirror_user_agent;
mod mirror_custom_headers;
mod middleware_message;

use axum::{http::Method, routing::{get, post}, Extension, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::routes::{hello_world::hello_world, middleware_message::middleware_message, mirror_body_json::mirror_body_json, mirror_body_string::mirror_body_string, mirror_custom_headers::mirror_custom_header, mirror_user_agent::mirror_user_agent, path_variables::{hard_coded_path, path_variables}, query_params::query_params};

#[derive(Debug, Clone)]
pub struct SharedData {
    pub message: String, 
}

pub async fn create_routes() -> Router  {

    let cors = CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(Any);
    let shared_data = SharedData {message: String::from("Hello from shared data!")};

    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/{:id}", get(path_variables))
        .route("/path_variables/15", get(hard_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(cors)
        .layer(Extension(shared_data))
        // .with_state(state)
}

