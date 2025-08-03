use axum::{http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use serde::{Deserialize, Serialize};

use crate::database::users;

#[derive(Deserialize)]
pub struct RequestUser {
    username: String, 
    password: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ResponseUser {
    username: String,
    id: i32,
    token: String
}

pub async fn create_user(
    Extension(database): Extension<DatabaseConnection>,
    Json(requesr_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let new_user = users::ActiveModel {
        username: Set(requesr_user.username),
        password: Set(requesr_user.password),
        token: Set(Some("bla123123asdf".to_string())),
        ..Default::default()
    }.save(&database)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let resp_user = Json(ResponseUser {
        username: new_user.username.unwrap(),
        id: new_user.id.unwrap(),
        token: new_user.token.unwrap().unwrap(),
    });
    Ok(resp_user)
}