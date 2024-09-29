use axum::{extract::{Path, Query}, http::StatusCode, Extension, Json};
use sea_orm::{ prelude::DateTimeWithTimeZone, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use crate::database::{todos::{self, Entity as todo}, users};
#[derive(Serialize, Deserialize)]
pub struct ResponseTodo{
    id:i32,
    description: Option<String>,
    category: Option<String>,
    done: Option<bool>,
    date : Option<String>,
    name: String
}

pub async fn get_one_task(
    Path(todo_id):Path<i32>,
     Extension(database):Extension<DatabaseConnection>
)-> Result<Json<ResponseTodo>, StatusCode>{
    let res_todo = todo::find_by_id(todo_id).one(&database).await.unwrap();
    if let Some(r_todo) = res_todo{
         Ok(Json(ResponseTodo{
            id:r_todo.id,
            name:r_todo.name,
            description:r_todo.description,
            category:r_todo.category,
            done:r_todo.done,
            date:r_todo.date,
        }))
    }else{
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_all_todo(
    Extension(database):Extension<DatabaseConnection>,
    Extension(user): Extension<users::Model>,
)->Result<Json<Vec<ResponseTodo>>, StatusCode>{
    let todos = todo::find()
        .filter(todos::Column::UserId.eq(user.id))
        .all(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|r_todo| ResponseTodo{
            id:r_todo.id,
            name:r_todo.name,
            description:r_todo.description,
            category:r_todo.category,
            done:r_todo.done,
            date:r_todo.date,
        })
        .collect();
    Ok(Json(todos)) 
}