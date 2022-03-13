use std::io::prelude::*;
use std::net:: {
	Shutdown,
	TcpListener,
	TcpStream,
};

use super::utils::parser;
use super::utils::thread_pool::ThreadPool;

pub struct Listener {
	thread_pool: ThreadPool,
}

impl Listener {
    pub fn new(pool_size: usize) -> Listener {
		Listener {
			thread_pool: ThreadPool::new(pool_size)
		}
	}

	pub fn init(&self, addr: &String) {
		let listener = match TcpListener::bind(addr) {
			Ok(x) => x,
			Err(e) => panic!("{:?}", e)
		};

		println!("Started listening on {:?}", addr);

		for stream in listener.incoming().take(2) {
			self.handle_connection( stream.unwrap())
		}
	}

	fn handle_connection(&self, stream: TcpStream) {
		self.thread_pool.execute(move || {
			let mut stream = stream;

			let mut buf: Vec<u8> = Vec::with_capacity(500);
			stream.read(&mut buf).unwrap();
			let msg = parser::parse_message(
				Box::new(String::from_utf8(buf).unwrap()));

			println!("{:?}", msg.data);

			stream.shutdown(Shutdown::Both).unwrap();
		})
	}
}