use base64:: {
	encode,
	decode
};

pub fn prep_message(data: Box<String>) -> Box<Message> {
	// 1. compress
	// 2. encrypt
	// 3. encode -- done

	let msg = Message::new(data).unwrap();
	encode(&msg.data);

	Box::new(msg)
}

pub fn parse_message(data: Box<String>) -> Box<Message> {
	let mut msg = Message::new(data).unwrap();
	match decode(&msg.data) {
		Ok(decoded) => msg.data = decoded,
		Err(e) => panic!("{:?}", e)
	};

	Box::new(msg)
}

pub struct Message {
	pub data: Vec<u8>
}

impl Message {
	pub fn new(data: Box<String>) -> Result<Message, String> {
		let max = 500;
		let data = data.as_bytes().to_vec();

		if data.len() < max { 
			return Ok(Message { data })
		} else {
			return Err(format!(
				"The message exceeds maximum length of {} characters",
				max));
		};
	}
}