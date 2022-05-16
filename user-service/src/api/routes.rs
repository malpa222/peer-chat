use super::*;
use crate::models::user::*;
use auth_helper;

use actix_web::{
    get,
    post,
    patch,
    delete,
    Result,
    Responder,
    web::{self, Json}, 
    error::{ErrorUnauthorized},
};

#[post("/users/post/")]
pub async fn add_user(user: Json<ApiUser>) -> Result<impl Responder> {
    let result = db_helper::add_user(&user).await;
    Ok(Json(result.unwrap()))
}

#[patch("/users/patch/")]
pub async fn update_user(user: Json<ApiUser>) -> Result<impl Responder> {
    let result = db_helper::update_user(&user).await;
    Ok(Json(result.unwrap()))
}

#[get("/users/get/{user_id}")]
pub async fn get_user(path: web::Path<i32>) -> Result<impl Responder> {
    match db_helper::get_user(path.into_inner()).await {
        Ok(res) => match auth_helper::get_user_meta(&res[0].email).await {
            Ok(meta) => Ok(Json(meta)),
            Err(err) => Err(ErrorUnauthorized(err))
        },
        Err(err) => Err(ErrorUnauthorized(err))
    }
}

#[delete("/users/delete/{user_id}")]
pub async fn delete_user(path: web::Path<i32>) -> Result<impl Responder> {
    let result = db_helper::delete_user(path.into_inner()).await;
    Ok(Json(result.unwrap()))
}