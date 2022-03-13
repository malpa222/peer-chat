use std::thread;
use std::sync:: {
	mpsc,
	Arc,
	Mutex
};

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
	NewJob(Job),
	Terminate
}

pub struct ThreadPool {
	workers: Vec<Worker>,
	sender: mpsc::Sender<Message>
}

impl ThreadPool {
	pub fn new(size: usize) -> ThreadPool {
		if size <= 0 {
			panic!("The pool size must be bigger than 0!");
		}

		let (sender, receiver) = mpsc::channel();
		let receiver = Arc::new(Mutex::new(receiver));

		let mut workers = Vec::with_capacity(size);
		for _ in 0..size {
			workers.push(Worker::new(Arc::clone(&receiver)));
		}

		ThreadPool {
			workers,
			sender,
		}
	}

	pub fn execute<F>(&self, func: F)
		where
			F: FnOnce() + Send + 'static,
		{
			let job = Box::new(func);
			self.sender.send(
				Message::NewJob(job))
				.unwrap();
		}
}

impl Drop for ThreadPool {
	fn drop(&mut self) { 
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

		for worker in &mut self.workers {
			if let Some(thread) = worker.thread.take() {
				thread.join().unwrap();
			}
		}
	}
}

struct Worker {
	thread: Option<thread::JoinHandle<()>>
}

impl Worker {
	pub fn new(receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
		let thread = thread::spawn(move || loop {
			match receiver.lock().unwrap().recv().unwrap() {
				Message::NewJob(job) => {
					job()
				},
				Message::Terminate => break
			}
		});

		Worker { thread: Some(thread) }
	}
}