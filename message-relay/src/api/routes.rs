use crate::models::message::ApiMessage;

use super::*;

use actix_web::{
    get,
    post,
    Result,
    Responder,
    web::{self, Json},
};

#[get("/chats/get/{user_id}")]
pub async fn get_chats(path: web::Path<i32>) -> Result<impl Responder> {
    let chats = db_helper::get_user_chats(path.into_inner()).await;
    Ok(Json(chats.unwrap()))
}

#[get("/messages/get/{chat_id}")]
pub async fn get_messages(path: web::Path<i32>) -> Result<impl Responder> {
    let messages = db_helper::get_chat_messages(path.into_inner()).await;
    Ok(Json(messages.unwrap()))
}

#[post("/messages/post/")]
pub async fn send_message(msg: Json<ApiMessage>) -> Result<impl Responder> {
    let result = db_helper::create_message(&msg).await;
    Ok(Json(result.unwrap()))
}