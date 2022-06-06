use amiquip::{Connection, Exchange, Publish, Result};

use crate::models::user;

pub fn publish(user: &user::AuthUser) -> Result<()> {
    // Open connection.
    let mut connection = Connection::insecure_open("amqp://guest:guest@172.17.0.2:5672")?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Get a handle to the direct exchange on our channel.
    let exchange = Exchange::direct(&channel);

    // Publish a message to the "hello" queue.
    match serde_json::to_string(&user) {
        Ok(u) => {
            exchange.publish(
                Publish::new(u.as_bytes(), "users"))?;
        },
        Err(_) => panic!()
    }

    connection.close()
}