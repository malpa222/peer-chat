use amiquip::{Connection, Exchange, Publish, Result};

use crate::models::user;

pub fn publish(user: &user::AuthUser) -> Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@172.17.0.2:5672")?;

    let channel = connection.open_channel(None)?;
    let exchange = Exchange::direct(&channel);

    match serde_json::to_string(&user) {
        Ok(u) => {
            exchange.publish(
                Publish::new(u.as_bytes(), "users"))?;
        },
        Err(_) => panic!()
    }

    connection.close()
}