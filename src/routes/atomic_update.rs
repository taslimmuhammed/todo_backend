use axum::{extract::Path, http:: StatusCode, Extension, Json};
use sea_orm::{prelude::DateTimeWithTimeZone, DatabaseConnection, QueryFilter, Set};
use sea_orm::{EntityTrait, ColumnTrait};
use serde::Deserialize;
use crate::database::todo::{self, Entity as todo};
#[derive(Deserialize)]
pub struct RequestTask {
    pub priority: Option<String>,
    pub title: String,
    pub completed_at: Option<DateTimeWithTimeZone>,
    pub description: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub user_id: Option<i32>,
    pub is_default: Option<bool>,
}
pub async fn atomic_update(
    Extension(database):Extension<DatabaseConnection>,
    Path(task_id):Path<i32>,
    Json(request_task):Json<RequestTask>
)->Result<(),StatusCode>{
    let update_task = todo::ActiveModel{
        id: Set(task_id),
        priority: Set(request_task.priority),
        title: Set(request_task.title),
        completed_at: Set(request_task.completed_at),
        description: Set(request_task.description),
        deleted_at: Set(request_task.deleted_at),
        user_id: Set(request_task.user_id),
        is_default: Set(request_task.is_default),
    };
    todo::update(update_task)
        .filter(todo::Column::Id.eq(task_id))// if the eq is giving error import ColumnTrait
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}