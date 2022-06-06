use std::error::Error;

use super::*;
use diesel::{
    prelude::*,
    pg::PgConnection,
};
use crate::{schema, models::user::{User, ApiUser}};
use models::{
    message::{Message, ApiMessage},
    user_chat::UserChat,
};

fn establish_connection() -> Result<PgConnection, String> {
    match std::env::var("DATABASE_URL") { 
        Ok(var) => {
            match PgConnection::establish(&var) {
                Ok(conn) => Ok(conn),
                Err(err) => Err(err.to_string())
            }
        },
        Err(err) => Err(err.to_string())
    }
}

pub fn add_user(user: ApiUser) -> Result<usize, Box<dyn Error>> {
    use schema::users::dsl::*;
    let conn = establish_connection()?;

    let u = users
        .filter(email.like(&user.email))
        .load::<User>(&conn)?;

    if u.len() >= 1 {
        return Ok(0);
    }

    let rows = diesel::insert_into(users)
        .values(user)
        .execute(&conn);

    Ok(rows.unwrap())
}

pub async fn get_user_chats(userid: i32) -> Result<Vec<UserChat>, String> {
    use schema::user_chats::dsl::*;
    let conn = establish_connection()?;

    let rows = user_chats
        .filter(user_id.eq(userid))
        .load::<UserChat>(&conn);

    match rows {
        Ok(chats) => Ok(chats),
        Err(err) => Err(err.to_string())
    }
}

pub async fn get_chat_messages(chatid: i32) -> Result<Vec<Message>, String> {
    use schema::messages::dsl::*;
    let conn = establish_connection()?;

    let rows = messages
        .filter(chat_id.eq(chatid))
        .load::<Message>(&conn);

    match rows {
        Ok(msgs) => Ok(msgs),
        Err(err) => Err(err.to_string())
    }
}

pub async fn create_message(msg: &ApiMessage) -> Result<usize, String> {
    use schema::messages::dsl::*;
    let conn = establish_connection()?;

    let rows = diesel::insert_into(messages)
        .values(msg)
        .execute(&conn);

    Ok(rows.unwrap())
}