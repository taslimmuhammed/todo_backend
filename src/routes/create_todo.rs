use axum::{http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use crate::database::{todos, users::Model};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTodo{
    name:String,
    description: Option<String>,
    category: Option<String>,
    date : Option<String>
}
pub async fn create_todo(
    Extension(database):Extension<DatabaseConnection>,
    Extension(user): Extension<Model>,
    Json(request_todo):Json<RequestTodo>,
)-> Result<(), StatusCode>{
        let task = todos::ActiveModel{
        user_id:Set(Some(user.id)), 
        name:Set(request_todo.name),
        description:Set(request_todo.description),
        category:Set(request_todo.category),
        done:Set(Some(false)),
        date:Set(request_todo.date),
        ..Default::default() //for auto increment id
    };
    let result = task.save(&database).await.unwrap();
    dbg!(result);
    Ok(())
}

// fn remove_bearer_prefix(token: &str) -> &str {
//     token.trim_start_matches("Bearer ").trim()
// }