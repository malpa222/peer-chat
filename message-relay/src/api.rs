use actix_web::{get, post, web, HttpResponse, Responder};
use entity::{message, user};

#[get("/msg/{user_id}")]
pub async fn get_messages(path: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("messages for {}", path.into_inner()))
}

#[post("/msg/{user_id}")]
pub async fn send_message(path: web::Path<String>, msg: web::Json<String>) -> impl Responder {
    HttpResponse::Ok()
        .body(format!("id: {}\nbody: {}", path.into_inner(), msg))
}