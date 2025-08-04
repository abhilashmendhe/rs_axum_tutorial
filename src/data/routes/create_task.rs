use axum::http::StatusCode;
use axum::{Extension, Json};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use serde::Deserialize;

use crate::database::{tasks, users};
use crate::database::users::Entity as Users;

#[derive(Deserialize)]
pub struct RequestTask {
    title: String,
    priority: Option<String>,
    description: Option<String>
}

pub async fn create_task(
    Extension(database): Extension<DatabaseConnection>,
    authorization: TypedHeader<Authorization<Bearer>>,
    Json(request_task): Json<RequestTask>,
) -> Result<(), StatusCode> {

    let token = authorization.token();

    let user = if let Some(user) = Users::find()
                .filter(users::Column::Token.eq(Some(token)))
                .one(&database)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
                    user
                } else {
                    return Err(StatusCode::UNAUTHORIZED);
                };
    
    let new_task = tasks::ActiveModel {
        priority: Set(request_task.priority),
        title: Set(request_task.title),
        description: Set(request_task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    let _result = new_task.save(&database).await.unwrap();

    // dbg!(result);
    Ok(())
}