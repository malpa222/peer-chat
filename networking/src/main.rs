use std::io;
use server::Server;

fn main() {
	let server = Server::new();
	server.init(&String::from("127.0.0.1:5000"));
}