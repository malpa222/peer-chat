use super::*;
use crate::schema;
use crate::models::{
    user::User,
    user_chat::UserChat,
};

pub async fn establish_connection(url: &str) -> PgConnection {
    PgConnection::establish(url)
        .expect(&format!("Error connecting to {}", url))
}

pub async fn get_user_chats(conn: &PgConnection, user: &User) -> Result<Vec<i32>, String> {
    use schema::user_chats::dsl::*;

    let rows = user_chats
        .filter(user_id.eq(user.id))
        .select(chat_id)
        .load::<i32>(conn);

    match rows {
        Ok(chats) => Ok(chats),
        Err(err) => Err(err.to_string())
    }
}

pub async fn create_user() {
    todo!()
}

pub async fn update_user() {
    todo!()
}

pub async fn delete_user() {
    todo!()
}
// pub async fn connect_to_db(url: String) -> DatabaseConnection {
//     let mut opt = ConnectOptions::new(url);
//     opt.max_connections(100)
//         .min_connections(5)
//         .connect_timeout(Duration::from_secs(8))
//         .idle_timeout(Duration::from_secs(8))
//         .max_lifetime(Duration::from_secs(8))
//         .sqlx_logging(true);
//         
//     match Database::connect(opt).await {
//         Ok(db) => db,
//         Err(err) => panic!("Cannot connect to database!\n{:?}", err)
//     }
// }
// 
// pub async fn get_user_chats(db: &DatabaseConnection, user_id: &str) -> Result<Vec<(user_chat::Model, Vec<chat::Model>)>, DbErr> {
//     let chats: Vec<(user_chat::Model, Vec<chat::Model>)> = UserChat::find()
//         .filter(user_chat::Column::UserId.eq(user_id))
//         .find_with_related(Chat)
//         .all(db)
//         .await?;
// 
//     return Ok(chats);
// }
// 
// pub async fn get_messages_from_chat(db: &DatabaseConnection, chat_id: &str) -> Result<Vec<message::Model>, DbErr> {
//     let msgs = Message::find()
//         .filter(message::Column::ChatId.eq(chat_id))
//         .all(db)
//         .await?;
// 
//     return Ok(msgs);
// }
// 
// pub async fn send_message(db: &DatabaseConnection, msg: message::Model) -> Result<message::ActiveModel, DbErr> {
//     let message = message::ActiveModel {
//         id: ActiveValue::Set(Uuid::new_v4()),
//         user_id: ActiveValue::Set(msg.user_id),
//         chat_id: ActiveValue::Set(msg.chat_id),
//         payload: ActiveValue::Set(msg.payload),
//         created_at: ActiveValue::Set(Some(Utc::now())),
//         updated_at: ActiveValue::Set(Some(Utc::now())),
//     };
// 
//     return Ok(message);
// }
// 
// pub async fn create_chat(chat: chat::Model) {
//     todo!()
// }