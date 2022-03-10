extern crate dotenv;

use dotenv::dotenv;
use std::env;

use server::Server;

fn main() {
	dotenv().ok();

	let server = Server::new(8);
	server.init(&String::from(env::var("SERV_ADDR").unwrap()));
}