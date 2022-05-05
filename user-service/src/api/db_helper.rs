use super::*;
use diesel::{
    prelude::*,
    pg::PgConnection,
};
use crate::schema;
use models::user::*;

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

pub async fn get_user(user_id: i32) -> Result<Vec<User>, String> {
    use schema::users::dsl::*;
    let conn = establish_connection()?;

    let result = users
        .find(user_id)
        .load::<User>(&conn);

    match result {
        Ok(user) => Ok(user),
        Err(err) => Err(err.to_string())
    }
}

pub async fn update_user(user: &ApiUser) -> Result<User, String> {
    use schema::users::dsl::*;
    let conn = establish_connection()?;

    let result = diesel::update(
        users.filter(email.eq(&user.email)))
        .set((email.eq(&user.email), username.eq(&user.username)))
        .get_result(&conn);

    match result {
        Ok(res) => Ok(res),
        Err(err) => Err(err.to_string())
    }
}

pub async fn add_user(user: &ApiUser) -> Result<usize, String> {
    use schema::users::dsl::*;
    let conn = establish_connection()?;

    let rows = diesel::insert_into(users)
        .values(user)
        .execute(&conn);

    Ok(rows.unwrap())
}

pub async fn delete_user(user_id: i32) -> Result<usize, String> {
    use schema::users::dsl::*;
    let conn = establish_connection()?;

    let rows = diesel::delete(users.find(user_id))
        .execute(&conn);

    Ok(rows.unwrap())
}