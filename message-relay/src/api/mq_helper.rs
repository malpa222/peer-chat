use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result};
use serde_json::{Value, from_str};

use crate::models::user::ApiUser;

use super::db_helper;

pub fn consume() -> Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@172.17.0.2:5672")?;

    let channel = connection.open_channel(None)?;
    let queue = channel.queue_declare("users", QueueDeclareOptions::default())?;

    let consumer = queue.consume(ConsumerOptions::default())?;
    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                
                let u: Value = from_str(&body).unwrap();
                println!("({:>3}) Received [{}]", i, body);

                db_helper::add_user(ApiUser { email: u["username"].to_string().replace("\"", "") }).unwrap();

                consumer.ack(delivery)?;
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()
}