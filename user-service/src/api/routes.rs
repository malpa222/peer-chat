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
    web::{self, Json, Query}, 
    error::{ErrorNotFound, ErrorInternalServerError, ErrorBadRequest},
};
use serde::Deserialize;

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


#[derive(Deserialize)]
pub struct GetUserQ {
    id: Option<i32>,
    auth0_id: Option<String>
}
#[get("/users/get")]
pub async fn get_user(q: Query<GetUserQ>) -> Result<impl Responder> {
    if let Some(id) = q.id {
        let row = db_helper::get_user(id)? // check if user exists in local db
            .into_iter()
            .next();

        match row {
            Some(user) => { // exists in local db
                match auth_helper::get_user_by_email(&user.email).await? { // check if user exists in auth0
                    Some(auth0) => return Ok(Json(ApiUser { email: auth0.email, username: auth0.username })), // exists. return ApiUser
                    None => { // does not exist. try to delete user from database
                        match db_helper::delete_user(id).await {
                            Ok(_) => Err(ErrorNotFound("This user does not exist")), // Delete was ok
                            Err(_) => Err(ErrorInternalServerError("Problem with retrieving data. Try again.")) // trouble deleting
                        }
                    },
                }
            },
            None => { // does not exist in local db
                if let Some(auth_id) = &q.auth0_id {
                    match auth_helper::get_user_by_id(auth_id).await? { // check if user exists in auth0
                        Some(user) => { // exists
                            match db_helper::add_user_auth(&user).await {
                                Ok(_) => return Ok(Json(ApiUser {
                                    email: user.email,
                                    username: user.username,
                                })), // Delete was ok
                                Err(_) => return Err(ErrorInternalServerError("Problem with registration. Try again.")) // trouble adding user
                            }
                        },
                        None => return Err(ErrorNotFound("This user does not exist")) // does not exist
                    }
                }

                Err(ErrorNotFound("Cannot find user"))
            },
        }
    } else {
        return Err(ErrorBadRequest("User id is missing"));
    }
}

#[delete("/users/delete/{user_id}")]
pub async fn delete_user(path: web::Path<i32>) -> Result<impl Responder> {
    let result = db_helper::delete_user(path.into_inner()).await;
    Ok(Json(result.unwrap()))
}