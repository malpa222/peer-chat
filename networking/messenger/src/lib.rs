fn send_message(data: &Box<String>) -> Result<String, ()> {
	unimplemented!();
}

fn encrypt_message(data: &Box<String>) -> Result<String, ()> {
	unimplemented!();
}

fn compress_message(data: &Box<String>) -> Result<String, ()> {
	unimplemented!();
}

fn encode_message(data: &Box<String>) -> Result<String, ()> {
	unimplemented!();
}

struct Message {
	header: String,
	data: Box<String>,
	footer: String,
}

impl Message {
	pub fn new(data: Box<String>) {
		unimplemented!();
	}

	fn build_header() {
		unimplemented!();
	}

	fn build_footer() {
		unimplemented!();
	}
}
