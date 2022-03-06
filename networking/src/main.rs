use server::Server;

fn main() {
	let server = Server::new(8);
	server.init(&String::from("127.0.0.1:5000"));
}