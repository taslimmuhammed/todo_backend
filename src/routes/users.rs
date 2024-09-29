use axum::{http::{HeaderMap, StatusCode}, Extension, Json};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,IntoActiveModel, Set};
use serde::{Deserialize, Serialize};

use crate::{database::users, utils::{custom_error::CustomError, jwt::create}};

#[derive(Deserialize, Debug, Clone)]
pub struct RequestAccount{
    firstname: String,
    lastname: String,
    email: String,
    password: String 
}
#[derive(Deserialize, Debug, Clone)]
pub struct LoginAccount{
    email: String,
    password: String 
}
#[derive(Serialize)]
pub struct ResponseAccount{
    id:i32,
    email: String,
    firstname: String,
    lastname: String,
    token: String
}
pub async fn create_account(
    Extension(database):Extension<DatabaseConnection>,
    Json(account):Json<RequestAccount>
)->Result<Json<ResponseAccount>, CustomError>{
    dbg!(account.clone());
    let new_user = users::ActiveModel{
        firstname: Set(account.firstname),
        lastname: Set(account.lastname),
        email: Set(account.email),
        password: Set(hash_password(account.password)?),
        token: Set(Some(create().map_err(|_|CustomError::new("error creating token",StatusCode::INTERNAL_SERVER_ERROR))?)),
        ..Default::default()
    }.save(&database)
    .await
    .map_err(|e| CustomError::new(e.to_string(),StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(
        Json(
            ResponseAccount { 
                id: new_user.id.unwrap(),
                firstname: new_user.firstname.unwrap(),
                lastname: new_user.lastname.unwrap(),
                email: new_user.email.unwrap(),
                token: new_user.token.unwrap().unwrap()
            }
        )
    )
}

pub async fn login_user(
    Extension(database):Extension<DatabaseConnection>,
    Json(account):Json<LoginAccount>
)->Result<Json<ResponseAccount>, CustomError>{
    let db_user = users::Entity::find()
        .filter(users::Column::Email.eq(account.email))
        .one(&database)
        .await
        .map_err(|_| CustomError::new("internal server error",StatusCode::INTERNAL_SERVER_ERROR))?;
    if let Some(db_user) = db_user{
        if !verify_password(account.password, &db_user.password)?{
            return Err(CustomError::new("wrong password", StatusCode::UNAUTHORIZED));
        }
        println!("user found");
        let new_token = create().map_err(|e| CustomError::new(e.as_str(), StatusCode::INTERNAL_SERVER_ERROR))?;
        let mut user = db_user.into_active_model();
        user.token = Set(Some(new_token));
        let saved_user = user.save(&database)
            .await
            .map_err(|e| CustomError::new("internal server error",StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(Json(ResponseAccount{
            id: saved_user.id.unwrap(),
            firstname: saved_user.firstname.unwrap(),
            lastname: saved_user.lastname.unwrap(),
            email: saved_user.email.unwrap(),
            token: saved_user.token.unwrap().unwrap()
        }))
    }else{
        Err(CustomError::new("Wrong username",StatusCode::NOT_FOUND))
    }
    

}

pub async fn logout(
    headers: HeaderMap,
    Extension(database):Extension<DatabaseConnection>
)-> Result<(), StatusCode>{
    let token = headers.get("authorization").unwrap().to_str().unwrap();
    let token =token.trim_start_matches("Bearer ").trim();
    let mut user = if let Some(user) = users::Entity::find()
    .   filter(users::Column::Token.eq(token))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?{
            user.into_active_model()
        }else{
            return  Err(StatusCode::UNAUTHORIZED);
        };
    user.token = Set(None);
    let _ = user.save(&database).
        await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    Ok(())
}

pub fn hash_password(password:String)->Result<String, CustomError>{
    bcrypt::hash(password, 10).map_err(|_| CustomError::new("error hashing",StatusCode::INTERNAL_SERVER_ERROR))
}

pub fn verify_password(password:String, hash:&str)-> Result<bool, CustomError>{
    bcrypt::verify(password, &hash).map_err(|_| CustomError::new("wrong pass", StatusCode::INTERNAL_SERVER_ERROR))
}