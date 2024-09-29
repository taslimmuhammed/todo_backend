use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{database::users, utils::{custom_error::CustomError, jwt}};

pub async fn guard(mut request:Request, next:Next)-> Result<Response, CustomError>{
    let token = request.headers()
        .get("authorization")
        .ok_or(CustomError::new("token not found", StatusCode::UNAUTHORIZED))?
        .to_str()
        .unwrap()
        .trim_start_matches("Bearer ")
        .trim()
        .to_owned();
    let database = request
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or(CustomError::new("server error", StatusCode::INTERNAL_SERVER_ERROR))?;

    let user = users::Entity::find()
        .filter(users::Column::Token.eq(token.clone()))
        .one(database)
        .await
        .map_err(|_| CustomError::new("server error", StatusCode::INTERNAL_SERVER_ERROR))?;
    jwt::is_valid(&token)?; //use verfification after checking database since that is always faster, good against attackers
    let Some(user)  = user else {
        return Err(CustomError::new("token expired", StatusCode::UNAUTHORIZED))
    };

    request.extensions_mut().insert(user);
    Ok(next.run(request).await)

}