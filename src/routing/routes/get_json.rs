use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Data {
    message: String,
    count: i32,
    username: String
}
pub async fn get_json() -> Json<Data> {
    let data = Data {
        message: "Hi. I am JSON data!".to_string(),
        count: 123,
        username: "abhi".to_string()
    };
    Json(data)
}