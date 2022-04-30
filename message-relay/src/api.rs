// use super::db_helper;
// 
// use actix_web::{get, post, web, web::Json, HttpResponse, Responder};

// #[get("/chats/get/{user_id}")]
// pub async fn get_chats(state: web::Data<State>, path: web::Path<String>) -> Json<String> {
//     let chats = db_helper::get_user_chats(
//         &state.db_con,
//         path.into_inner().as_str()).await.unwrap();
// 
//     Json(String::from("123"))
// }
// 
// #[get("/messages/get/{chat_id}")]
// pub async fn get_messages(db: web::Data<DatabaseConnection>, path: web::Path<String>) -> impl Responder {
//     HttpResponse::Ok().body(format!("messages for {}", path.into_inner()))
// }
// 
// #[post("/messages/post/{user_id}")]
// pub async fn send_message(db: web::Data<DatabaseConnection>, path: web::Path<String>, msg: web::Json<String>) -> impl Responder {
//     HttpResponse::Ok()
//         .body(format!("id: {}\nbody: {}", path.into_inner(), msg))
// }