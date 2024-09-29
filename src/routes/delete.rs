use axum::{extract::Path, http::StatusCode, Extension};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};

use crate::database::todo;

pub async fn delete_task(
    Path(task_id):Path<i32>,
    Extension(database):Extension<DatabaseConnection>
)->Result<(),StatusCode>{
    // let task = if let Some(task) = todo::Entity::find_by_id(task_id)
    //     .one(&database)
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?{
    //         task.into_active_model()
    //     }else{ 
    //         return Err(StatusCode::NOT_FOUND)
    //     };
    // todo::Entity::delete(task)
    //     .exec(&database)
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // Ok(())
    todo::Entity::delete_many()
        .filter(todo::Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

pub async fn soft_delete(
    Path(task_id):Path<i32>,
    Extension(database):Extension<DatabaseConnection>
)->Result<(),StatusCode>{
    let mut task = if let Some(task) = todo::Entity::find_by_id(task_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?{
            task.into_active_model()
        }else{
            return Err(StatusCode::NOT_FOUND);
        };
    let now = chrono::Utc::now();
    task.deleted_at = Set(Some(now.into()));
    todo::Entity::update(task)
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}