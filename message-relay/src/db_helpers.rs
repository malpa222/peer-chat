use sea_orm::{Database, DatabaseConnection, ConnectOptions};
use std::time::Duration;

pub async fn connect_to_db(url: String) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);
        
    match Database::connect(opt).await {
        Ok(db) => db,
        Err(err) => panic!("Cannot connect to database!\n{:?}", err)
    }
}

pub async fn get_user_messages() {

}

pub async fn send_message() {

}