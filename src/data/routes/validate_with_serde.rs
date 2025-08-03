use axum::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct RequestUser {
    // #[validate(email(message="must be a valid email"))]
    pub username: String,
    // #[validate(length(min=8,message="must have atleast 8 characters"))]
    pub password: String
}

pub async fn validate_with_serde(Json(user): Json<RequestUser>) {
    dbg!(user);
}