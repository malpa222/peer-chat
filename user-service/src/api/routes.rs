use super::*;
use crate::models::user::User;

use actix_web::{
    get,
    post,
    patch,
    delete,
    Result,
    Responder,
    web::{self, Json},
};

#[post("/users/post/")]
pub async fn add_user(user: Json<User>) -> Result<impl Responder> {
    let result = db_helper::add_user(&user).await;
    Ok(Json(result.unwrap()))
}

#[patch("/users/patch/")]
pub async fn update_user(user: Json<User>) -> Result<impl Responder> {
    let result = db_helper::update_user(&user).await;
    Ok(Json(result.unwrap()))
}

#[get("/users/get/{user_id}")]
pub async fn get_user(path: web::Path<i32>) -> Result<impl Responder> {
    let result = db_helper::get_user(path.into_inner()).await;
    Ok(Json(result.unwrap()))
}

#[delete("/users/delete/{user_id}")]
pub async fn delete_user(path: we::Path<i32>) -> Result<impl Responder> {
    let result = db_helper::delete_user(path.into_inner()).await;
    Ok(Json(result.unwrap()))
}