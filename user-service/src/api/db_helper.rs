use super::*;
use diesel::{
    prelude::*,
    pg::PgConnection,
};
use crate::schema;
use models::user::User;

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

pub async fn get_user(userid: i32) -> Result<User, String> {
    use schema::users::dsl::*;
    let conn = establish_connection()?;

    let result = users
        .filter(user_id.eq(userid))
        .load::<UserChat>(&conn);

    match rows {
        Ok(chats) => Ok(chats),
        Err(err) => Err(err.to_string())
    }
}

pub async fn update_user(chatid: i32) -> Result<Vec<Message>, String> {
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

pub async fn add_user(msg: &ApiMessage) -> Result<usize, String> {
    use schema::messages::dsl::*;
    let conn = establish_connection()?;

    let rows = diesel::insert_into(messages)
        .values(msg)
        .execute(&conn);

    Ok(rows.unwrap())
}

pub async fn delete_user(msg: &ApiMessage) -> Result<usize, String> {
    use schema::messages::dsl::*;
    let conn = establish_connection()?;

    let rows = diesel::insert_into(messages)
        .values(msg)
        .execute(&conn);

    Ok(rows.unwrap())
}