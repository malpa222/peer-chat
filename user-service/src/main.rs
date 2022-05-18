#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use actix_web::{App, HttpServer, middleware::Logger};

pub mod api;
pub mod models;
pub mod schema;

use api::routes;

macro_rules! getenv {
    ($a:expr) => {
        std::env::var($a).expect(format!("{} is not defined", $a).as_str())
    };
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    dotenv().ok();

    let ip = format!("{}:{}", getenv!("SERVER_ADDR"), getenv!("SERVER_PORT"));
    println!("Server starting at: {}", ip);

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .service(routes::get_user)
            .service(routes::update_user)
            .service(routes::delete_user)
    })
    .bind(ip)?
    .run()
    .await
}