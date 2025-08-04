use axum::{http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::database::users::{self, Entity as Users};

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
    
    dbg!(&resp_user);
    Ok(resp_user)
}

pub async fn login(
    Extension(database): Extension<DatabaseConnection>,
    Json(requesr_user): Json<RequestUser>
) -> Result<Json<ResponseUser>, StatusCode> {

    let db_user = Users::find()
                .filter(users::Column::Username.eq(&requesr_user.username))
                .one(&database)
                .await
                .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if let Some(db_user) = db_user {
        let new_token = "asdf124312asdfasdf123aretlih".to_owned();
        let mut user = db_user.into_active_model();

        user.token = Set(Some(new_token));

        let saved_user = user.save(&database)
                        .await
                        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(ResponseUser {
            username: saved_user.username.unwrap(),
            id: saved_user.id.unwrap(),
            token: saved_user.token.unwrap().unwrap(),
        }))

    } else {
        Err(StatusCode::NOT_FOUND)
    }
}