#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use actix_web::{App, HttpServer, middleware::Logger};
use actix_web_prom::PrometheusMetricsBuilder;
use actix_cors::*;

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
    dotenv().ok();
    env_logger::init();

    let ip = format!("{}:{}", getenv!("SERVER_ADDR"), getenv!("SERVER_PORT"));
    println!("Server starting at: {}", ip);

    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .wrap(prometheus.clone())
            .service(routes::get_user)
            .service(routes::update_user)
            .service(routes::delete_user)
    })
    .bind(ip)?
    .run()
    .await
}