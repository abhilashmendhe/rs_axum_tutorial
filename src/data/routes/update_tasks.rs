use axum::http::StatusCode;
use axum::{extract::Path, Extension, Json};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::{prelude::DateTimeWithTimeZone, ActiveValue::Set, DatabaseConnection};
use serde::Deserialize;
use crate::database::tasks::Entity as Tasks;

use crate::database::tasks;

#[derive(Deserialize)]
pub struct RequestTask {
    pub id: Option<i32>,
    pub priority: Option<String>,
    pub title: String,
    pub completed_at: Option<DateTimeWithTimeZone>,
    pub description: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub user_id: Option<i32>,
    pub is_default: Option<bool>,
}

pub async fn atomic_update(
    Path(task_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(req): Json<RequestTask>
) -> Result<(), StatusCode> {
    let update_task = tasks::ActiveModel {
        id: Set(task_id),
        priority: Set(req.priority),
        title: Set(req.title),
        completed_at: Set(req.completed_at),
        description: Set(req.description),
        deleted_at: Set(req.deleted_at),
        user_id: Set(req.user_id),
        is_default: Set(req.is_default) 
    };

    Tasks::update(update_task)
        .filter(tasks::Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(())
}