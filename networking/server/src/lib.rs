extern crate base64;

use std::io::prelude::*;
use std::net:: {
	Shutdown,
	TcpListener,
	TcpStream,
};

mod utils;
use utils::parser;
use utils::thread_pool::ThreadPool;

pub struct Server {
	thread_pool: ThreadPool,
}

impl Server {
	pub fn new(pool_size: usize) -> Server {
		Server {
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

	pub fn send_msg(&self, addr: &String, data: Box<String>) -> Result<(), String> {
		let mut stream = TcpStream::connect(addr).unwrap();
		let msg = *parser::prep_message(data);

		if stream.write(&msg.data).unwrap() > 0 {
			Ok(())
		} else {
			Err(String::from("Unable to send..."))
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