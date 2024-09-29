use axum::{extract::Path, http::StatusCode, Extension};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};

use crate::database::todos;

pub async fn delete_task(
    Path(task_id):Path<i32>,
    Extension(database):Extension<DatabaseConnection>
)->Result<(),StatusCode>{
    todos::Entity::delete_many()
        .filter(todos::Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}