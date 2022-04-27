use std::time::Duration;

use uuid::Uuid;

use sea_orm::{
    Database,
    DatabaseConnection,
    ConnectOptions,
    Condition,
    EntityTrait,
    QueryFilter,
    ColumnTrait,
    QueryOrder,
    DbErr,
    Related,
    ActiveValue,
};
use entity::{
    message,
    message::Entity as Message,
    user,
    user::Entity as User,
    chat,
    chat::Entity as Chat,
    user_chat,
    user_chat::Entity as UserChat,
};

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

pub async fn get_user_chats(db: DatabaseConnection, user_id: &str) -> Result<Vec<chat::Model>, DbErr> {
    let chats: Vec<(user_chat::Model, Vec<chat::Model>)> = UserChat::find()
        .filter(user_chat::Column::UserId.eq(user_id))
        .find_with_related(Chat)
        .all(&db)
        .await?;

    todo!()
}

pub async fn get_messages_from_chat(db: DatabaseConnection, chat_id: &str) -> Result<Vec<message::Model>, DbErr> {
    let msgs = Message::find()
        .filter(message::Column::ChatId.eq(chat_id))
        .all(&db)
        .await?;

    todo!()
}

pub async fn send_message(db: DatabaseConnection, msg: message::Model) -> Result<message::Model, DbErr> {
    let msg = Message::insert(msg).exec(&db).await?;

    todo!()
}

pub async fn create_chat(chat: chat::Model) {
    todo!()
}