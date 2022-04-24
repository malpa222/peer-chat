#[macro_use]
extern crate log;
extern crate dotenv;

use dotenv::dotenv;
use actix_web::{HttpServer, App};

mod api;
mod db_helpers;

macro_rules! envvar {
    ($a:expr) => {
        std::env::var($a).unwrap()
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    dotenv().ok();
    db_helpers::connect_to_db(envvar!("DATABASE_URL")).await;

    let ip = format!("{}:{}", envvar!("SERVER_ADDR"), envvar!("SERVER_PORT"));
    debug!("Server starting at: {}", ip);

    HttpServer::new(move || {
        App::new()
            .service(api::get_messages)
            .service(api::send_message)
    })
    .bind(ip)?
    .run()
    .await
}
