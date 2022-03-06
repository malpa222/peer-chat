pub mod thread_pool {
	pub struct ThreadPool {
		working: i32,
		max: usize
	}

	impl ThreadPool {
		pub fn new(size: usize, limit: Option<usize>) -> ThreadPool {
			match limit {
				Some(limit) => ThreadPool {
					working: 0,
					max: limit
				},
				None => ThreadPool {
					working: 0,
					max: size
				},
			}
		}

		pub fn spawn_thread(&mut self) {
			self.working += 1;
		}

		pub fn kill_thread(&mut self) {
			self.working += 1;
		}
	}
}