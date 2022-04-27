use actix_web::{get, post, web, HttpResponse, Responder};
use entity::{message, user};
use sea_orm::DatabaseConnection;

#[get("/msg/{user_id}")]
pub async fn get_messages(db: web::Data<DatabaseConnection>, path: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("messages for {}", path.into_inner()))
}

#[post("/msg/{user_id}")]
pub async fn send_message(db: web::Data<DatabaseConnection>, path: web::Path<String>, msg: web::Json<String>) -> impl Responder {
    HttpResponse::Ok()
        .body(format!("id: {}\nbody: {}", path.into_inner(), msg))
}