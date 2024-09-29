mod create_todo;
mod get_todo;
mod atomic_update;
// mod partial_update;
// mod delete;
mod users;
mod guard;
use guard::guard;
use users::{create_account, login_user, logout};
// use delete::{delete_task, soft_delete};
use create_todo::create_todo;
use get_todo::get_all_todo;
use atomic_update::atomic_update;
// use partial_update::partial_update;
use sea_orm::DatabaseConnection;
use axum::{
    http::{header::{ACCEPT, AUTHORIZATION, CONTENT_DISPOSITION, CONTENT_TYPE}, Method}, middleware, routing::{delete, get, patch, post, put}, Extension, Router
};
use tower_http::cors::{Any, CorsLayer};
pub fn create_routes(database:DatabaseConnection)-> Router{
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH, Method::OPTIONS])
    .allow_origin(["http://localhost:3000".parse().unwrap()])
    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
    .allow_credentials(true)
    .expose_headers([CONTENT_DISPOSITION]);
    let app = Router::new()
        .route("/create_todo", post(create_todo))
        .route("/get_all_todo", get(get_all_todo))
        .route("/update/:id", put(atomic_update))
        .layer(middleware::from_fn(guard))
        // .route("/get_one_task/:id", get(get_one_task))
        
        // .route("/partial_update/:task_id",patch(partial_update))
        // .route("/delete/:task_id", delete(delete_task))
        // .route("/soft_delete/:task_id", delete(soft_delete))
        .route("/auth/signup", post(create_account))
        .route("/auth/login", post(login_user))
        .route("/auth/logout", post(logout))
        .layer(Extension(database))
        .layer(cors);
    app
}