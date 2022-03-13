extern crate dotenv;

use dotenv::dotenv;
use std:: {
	env,
	io,
};

mod lib;
use lib:: {
	listener::Listener,
	sender,
	utils::thread_pool::ThreadPool
};

fn main() {
	dotenv().ok();

	let pool = ThreadPool::new(2);
	pool.execute(|| {
		let server = Listener::new(8);
		server
			.init(&String::from(env::var("SERV_ADDR")
			.unwrap()));
	});
}