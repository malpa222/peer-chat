use std::io::prelude::*;
use std::thread;
use std::time::Duration;

use std::net:: {
	TcpListener,
	TcpStream
};

mod thread_pool;

pub struct Server {
	connections: Vec<String>,
	thread_pool: thread_pool::ThreadPool,
}

impl Server {
	pub fn new() -> Server {
		Server {
			connections: Vec::new(),
			thread_pool: thread_pool::ThreadPool::new(size, limit)
		}
	}

	pub fn init(&self, addr: &String) {
		let listener = match TcpListener::bind(addr) {
			Ok(x) => x,
			Err(e) => panic!("{:?}", e)
		};

		println!("Started listening on {:?}", addr);

		for stream in listener.incoming() {
			self.handle_connection(stream.unwrap());
		}
	}

	pub fn deinit(&self) -> bool {
		unimplemented!();
	}

	fn handle_connection(&self, mut stream: TcpStream) {
		// let mut buf = [0; 1024];
		// stream.read(&mut buf).unwrap();
		// println!("Req: {}", String::from_utf8_lossy(&buf[..]));

		// stream.write(&res.as_bytes()).unwrap();
		// stream.flush().unwrap();

		unimplemented!();
	}

	fn parse_request(&self, req: Box<String>) -> bool {
		unimplemented!();
	}
}