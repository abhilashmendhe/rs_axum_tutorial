use axum::{extract::{Path, Query}, http::StatusCode, Extension};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, ActiveValue::Set};
use serde::Deserialize;
use crate::database::tasks::{self, Entity as Tasks};

#[derive(Deserialize)]
pub struct Params {
    soft: bool
}

pub async fn delete_task(
    Path(id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Query(qp): Query<Params>
) -> Result<(), StatusCode> {

    // 1. This is one way to do..
    // let task = if let Some(task) = Tasks::find_by_id(id)
    //             .one(&database)
    //             .await
    //             .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
    //                 task.into_active_model()
    //             } else {
    //                 return Err(StatusCode::NOT_FOUND);
    //             };
    // Tasks::delete(task)
    //     .exec(&database)
    //     .await
    //     .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 2. Second way
    // Tasks::delete_by_id(id)
    //     .exec(&database)
    //     .await
    //     .map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)?;

    if qp.soft {
        let mut task = if let Some(task) = Tasks::find_by_id(id)
                .one(&database)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? 
                {
                    task.into_active_model()
                } else {
                    return Err(StatusCode::NOT_FOUND);
                };
        let now = chrono::Utc::now();
        task.deleted_at = Set(Some(now.into()));
        Tasks::update(task)
                .exec(&database)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
    // 3. Delete many rows
    Tasks::delete_many()
        .filter(tasks::Column::Id.eq(id))
        .exec(&database)
        .await
        .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?;

    }
    Ok(())
}