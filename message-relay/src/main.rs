#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use actix_web::{App, HttpServer, middleware::Logger};

pub mod api;
pub mod repositories;
pub mod models;
pub mod schema;

macro_rules! envvar {
    ($a:expr) => {
        std::env::var($a).expect(format!("{} is not defined", $a).as_str())
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    dotenv().ok();
    // let db = db_helper::connect_to_db(envvar!("DATABASE_URL")).await;

    let ip = format!("{}:{}", envvar!("SERVER_ADDR"), envvar!("SERVER_PORT"));
    println!("Server starting at: {}", ip);

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            //.app_data(web::Data::new(State {
            //        db_con: db.clone() 
            //}))
            // .service(api::get_messages)
            // .service(api::send_message)
    })
    .bind(ip)?
    .run()
    .await
}