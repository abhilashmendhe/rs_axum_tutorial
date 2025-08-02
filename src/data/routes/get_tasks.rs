use axum::{extract::{Path, Query}, http::StatusCode, response::IntoResponse, Extension, Json};
use chrono::{DateTime, FixedOffset};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use crate::database::tasks::{self, Entity as Tasks};

#[derive(Serialize)]
pub struct ResponseTask {
    id: i32,
    title: String,
    priority: Option<String>,
    description: Option<String>,
    deleted_at: Option<DateTime<FixedOffset>>
}   

pub async fn get_one_task(
    Path(id): Path<i32>, 
    Extension(database): Extension<DatabaseConnection>
) -> impl IntoResponse {
    let task = Tasks::find_by_id(id)
    .filter(tasks::Column::DeletedAt.is_null())
                .one(&database)
                .await
                .unwrap();

    if let Some(task) = task {
            (
                StatusCode::OK,
                Json(ResponseTask {
                    id,
                    title: task.title,
                    priority: task.priority,
                    description: task.description,
                    deleted_at: task.deleted_at
                }
            )
        ).into_response()
    } else {
        (StatusCode::NOT_FOUND, "We couldn't find the task".to_string()).into_response()
    }
}

#[derive(Deserialize)]
pub struct GetTasksQueryParams {
    priority: Option<String>
}

pub async fn get_all_tasks(
    Extension(database): Extension<DatabaseConnection>,
    Query(qp): Query<GetTasksQueryParams>
) -> Result<Json<Vec<ResponseTask>>, StatusCode>{

    let priority_filter = if let Some(priority) = qp.priority {
        if priority.is_empty() {
            Condition::all().add(tasks::Column::Priority.is_null())
        } else {
            Condition::all().add(tasks::Column::Priority.eq(priority))
        }
    } else {Condition::all()};
    
    let tasks = Tasks::find()
                // .filter(tasks::Column::Priority.eq(qp.priority))
                .filter(priority_filter)
                .filter(tasks::Column::DeletedAt.is_null())
                .all(&database)
                .await
                .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?
                .into_iter()
                .map(|db_task| 
                    ResponseTask {
                        id: db_task.id,
                        title: db_task.title,
                        priority: db_task.priority,
                        description: db_task.description,
                        deleted_at: db_task.deleted_at
                    }
                ).collect::<Vec<_>>();
    
    Ok(Json(tasks))
}