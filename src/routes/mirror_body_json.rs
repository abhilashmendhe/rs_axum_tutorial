use axum::extract::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MirrorJson {
    message: String
}

#[derive(Debug, Serialize)]
pub struct MirrorJsonResponse {
    message: String,
    message_from_server: String
}

pub async fn mirror_body_json(Json(body): Json<MirrorJson>) -> Json<MirrorJsonResponse> {
    // dbg!(body);
    // Json(body)
    Json(MirrorJsonResponse { message: body.message, message_from_server: String::from("Hello from Axum!") })
}