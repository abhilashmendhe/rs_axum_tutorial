use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub async fn returns_201() -> Response {
    (
        StatusCode::CREATED, // 201
        "This is a 201".to_string()
    ).into_response()
}