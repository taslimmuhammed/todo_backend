mod create_todo;
mod get_todo;
// mod atomic_update;
// mod partial_update;
// mod delete;
mod users;
mod guard;
use guard::guard;
use users::{create_account, login_user, logout};
// use delete::{delete_task, soft_delete};
use create_todo::create_todo;
use get_todo::{ get_all_todo};
// use atomic_update::atomic_update;
// use partial_update::partial_update;
use sea_orm::DatabaseConnection;
use axum::{
    http::Method, middleware, routing::{delete, get, patch, post, put}, Extension, Router
};
use tower_http::cors::{Any, CorsLayer};
pub fn create_routes(database:DatabaseConnection)-> Router{
    let cors  = CorsLayer::new().allow_methods([Method::GET, Method::POST]).allow_origin(Any);
    let app = Router::new()
        .route("/create_todo", post(create_todo))
        .route("/get_all_todo", get(get_all_todo))
        .layer(middleware::from_fn(guard))
        // .route("/get_one_task/:id", get(get_one_task))
        // .route("/atomic_update/:task_id", put(atomic_update))
        // .route("/partial_update/:task_id",patch(partial_update))
        // .route("/delete/:task_id", delete(delete_task))
        // .route("/soft_delete/:task_id", delete(soft_delete))
        .route("/users/signup", post(create_account))
        .route("/users/login", post(login_user))
        .route("/users/logout", post(logout))
        // // .route("/test", post(test_func))
        .layer(Extension(database))
        .layer(cors);
    app
}