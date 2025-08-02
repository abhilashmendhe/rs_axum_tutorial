use axum::http::StatusCode;
use axum::{extract::Path, Extension, Json};
use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use sea_orm::{prelude::DateTimeWithTimeZone, ActiveValue::Set, DatabaseConnection};
use serde::Deserialize;
use crate::database::tasks::Entity as Tasks;

use crate::database::tasks;

#[derive(Deserialize)]
pub struct RequestTask {
    pub id: Option<i32>,
    #[serde(
        default,                                     // important for deserialization
        skip_serializing_if = "Option::is_none",     // important for serialization
        with = "::serde_with::rust::double_option"
    )]
    pub priority: Option<Option<String>>,
    
    pub title: Option<String>,
    #[serde(
        default,                                     // important for deserialization
        skip_serializing_if = "Option::is_none",     // important for serialization
        with = "::serde_with::rust::double_option"
    )]
    pub completed_at: Option<Option<DateTimeWithTimeZone>>,
    #[serde(
        default,                                     // important for deserialization
        skip_serializing_if = "Option::is_none",     // important for serialization
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    #[serde(
        default,                                     // important for deserialization
        skip_serializing_if = "Option::is_none",     // important for serialization
        with = "::serde_with::rust::double_option"
    )]
    pub deleted_at: Option<Option<DateTimeWithTimeZone>>,
    // #[serde(
    //     default,                                     // important for deserialization
    //     skip_serializing_if = "Option::is_none",     // important for serialization
    //     with = "::serde_with::rust::double_option"
    // )]
    // pub user_id: Option<Option<i32>>,
    // #[serde(
    //     default,                                     // important for deserialization
    //     skip_serializing_if = "Option::is_none",     // important for serialization
    //     with = "::serde_with::rust::double_option"
    // )]
    // pub is_default: Option<Option<bool>>,
}

/*
    For partial update we send HTTP PATCH request because it handles only partial updates.
    Let's say if we have a large JSON object, we can simply use PATCH to update certain fields by 
    sending a small JSON object
*/
pub async fn partial_update(
    Path(task_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(req): Json<RequestTask>
) -> Result<(), StatusCode> {
    let mut db_task = if let Some(task) = Tasks::find_by_id(task_id)
                .one(&database)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?{
                    task.into_active_model()
                } else {
                    return Err(StatusCode::NOT_FOUND);
                };
    
    if let Some(priority) = req.priority {
        db_task.priority = Set(priority);
    }

    if let Some(description) = req.description {
        db_task.description = Set(description);
    }

    if let Some(title) = req.title {
        db_task.title = Set(title);
    }

    if let Some(completed_at) = req.completed_at {
        db_task.completed_at = Set(completed_at);
    }

    if let Some(deleted_at) = req.deleted_at {
        db_task.deleted_at = Set(deleted_at);
    }

    Tasks::update(db_task)
        .filter(tasks::Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(())
}