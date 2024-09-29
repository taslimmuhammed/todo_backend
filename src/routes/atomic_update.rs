use axum::{extract::Path, http:: StatusCode, Extension, Json};
use sea_orm::{prelude::DateTimeWithTimeZone, DatabaseConnection, QueryFilter, Set};
use sea_orm::{EntityTrait, ColumnTrait};
use serde::Deserialize;
use crate::database::todos::{self, Entity as todo};
use crate::database::users;
#[derive(Deserialize)]
pub struct RequestTodo {
    id:i32,
    description: Option<String>,
    category: Option<String>,
    done: Option<bool>,
    date : Option<String>,
    name: String
}
pub async fn atomic_update(
    Extension(database):Extension<DatabaseConnection>,
    Extension(user): Extension<users::Model>,
    Path(id):Path<i32>,
    Json(request_todo):Json<RequestTodo>
)->Result<(),StatusCode>{
    let update_task = todos::ActiveModel{
        id: Set(id),
        user_id: Set(Some(user.id)),
        done:Set(request_todo.done),
        name:Set(request_todo.name),
        category:Set(request_todo.category),
        description:Set(request_todo.description),
        date:Set(request_todo.date),
    };
    todo::update(update_task)
        .filter(todos::Column::Id.eq(id))// if the eq is giving error import ColumnTrait
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}