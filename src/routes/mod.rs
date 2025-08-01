mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;
mod query_params;
mod mirror_user_agent;
mod mirror_custom_headers;
mod middleware_message;
mod read_middleware_custom_header;
mod set_middleware_custom_header;
mod always_errors;
mod returns_201;

use axum::{http::Method, middleware, routing::{get, post}, Extension, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::routes::{
    always_errors::always_errors, hello_world::hello_world, middleware_message::middleware_message, mirror_body_json::mirror_body_json, mirror_body_string::mirror_body_string, mirror_custom_headers::mirror_custom_header, mirror_user_agent::mirror_user_agent, path_variables::{hard_coded_path, path_variables}, query_params::query_params, read_middleware_custom_header::read_middleware_custom_header, returns_201::returns_201, set_middleware_custom_header::set_middleware_custom_header
};

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
        .route("/read_middleware_custom_header", get(read_middleware_custom_header))
        .route_layer(middleware::from_fn(set_middleware_custom_header))

        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/{:id}", get(path_variables))
        .route("/path_variables/15", get(hard_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(Extension(shared_data))
        .layer(cors)
        // make sure if anything after the layers wont' be able to access the data from the middleware to the routes
        .route("/always_errors", get(always_errors))
        .route("/returns_201", post(returns_201))
}

