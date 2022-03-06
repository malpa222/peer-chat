use std::net:: {
	TcpListener,
	TcpStream
};

mod thread_pool;
use self::thread_pool::ThreadPool;

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
			self.handle_connection(stream.unwrap())
		}
	}

	fn handle_connection(&self, stream: TcpStream) {
		self.thread_pool.execute(move || {
			println!("{:?}", stream);
		})
	}
}