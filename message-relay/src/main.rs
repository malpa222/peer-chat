#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::thread;
use dotenv::dotenv;
use actix_web::{App, HttpServer, middleware::Logger};
use actix_web_prom::PrometheusMetricsBuilder;

pub mod api;
pub mod models;
pub mod schema;

use api::routes;
use api::mq_helper;

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

    println!("Starting message consumer");
    thread::spawn(|| {
        mq_helper::consume().unwrap_or_else(|err| {
            panic!("{}", err);
        });
    });

    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();

    println!("Server starting at: {}", ip);
    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .wrap(prometheus.clone())
            .service(routes::get_chats)
            .service(routes::get_messages)
            .service(routes::send_message)
    })
    .bind(ip)?
    .run()
    .await
}