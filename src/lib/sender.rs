use std:: {
	io::prelude::*,
	net::TcpStream,
};

use super::utils::parser;

pub fn send_msg(addr: &String, data: Box<String>) -> Result<(), String> {
	let mut stream = TcpStream::connect(addr).unwrap();
	let msg = parser::prep_message(data);

	let written = stream.write(&msg.data)
		.expect("couldnt connect");

	if written  > 0 {
		Ok(())
	} else {
		Err(String::from("Unable to send..."))
	}
}