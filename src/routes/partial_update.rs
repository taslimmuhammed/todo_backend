use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{ ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use crate::database::todo;

#[derive(Deserialize, Serialize)]
struct Doc {
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    a: Option<Option<u8>>,
}
#[derive(Deserialize)]
pub struct RequestTask {
    // steps:-
    // Add option to all the values
    // Convert the values whcih will be updated
    // only do double option for values of option type, others just use normal option(eg.title here)
    pub title: Option<String>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub priority: Option<Option<String>>,

    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub description: Option<Option<String>>,
}
pub async fn partial_update(
    Extension(database):Extension<DatabaseConnection>,
    Path(task_id):Path<i32>,
    Json(request_task):Json<RequestTask>
)->Result<(), StatusCode>{
    let mut db_task = if let Some(task) = todo::Entity::find_by_id(task_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        task.into_active_model()
    }else{
        return Err(StatusCode::NOT_FOUND);
    };
    if let Some(priority) = request_task.priority{
        db_task.priority = Set(priority);
    }
    if let Some(title) = request_task.title{
        db_task.title = Set(title);
    }
    if let Some(description) = request_task.description{
        db_task.description = Set(description);
    }
    todo::Entity::update(db_task)
        .filter(todo::Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}