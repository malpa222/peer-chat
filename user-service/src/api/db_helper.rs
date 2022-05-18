use std::error::Error;

use super::*;
use diesel::{
    prelude::*,
    pg::PgConnection,
};

use crate::schema;
use models::user::*;

macro_rules! getenv {
    ($a:expr) => {
        std::env::var($a).expect(format!("{} is not defined", $a).as_str())
    };
}

fn establish_connection() -> Result<PgConnection, Box<dyn Error>> {
    match PgConnection::establish(&getenv!("DATABASE_URL")) {
        Ok(conn) => Ok(conn),
        Err(err) => Err(Box::from(err))
    }
}

pub fn get_user(user_id: i32) -> Result<Vec<User>, Box<dyn Error>> {
    use schema::users::dsl::*;
    let conn = establish_connection()?;

    let rows = users
        .find(user_id)
        .load::<User>(&conn);

    match rows {
        Ok(row) => Ok(row),
        Err(err) => Err(Box::from(err))
    }
}

pub async fn update_user(user: &ApiUser) -> Result<User, Box<dyn Error>> {
    use schema::users::dsl::*;
    let conn = establish_connection()?;

    let result = diesel::update(
        users.filter(email.eq(&user.email)))
        .set((email.eq(&user.email), username.eq(&user.username)))
        .get_result(&conn);

    match result {
        Ok(user) => Ok(user),
        Err(err) => Err(Box::from(err))
    }
}

pub async fn add_user(user: &ApiUser) -> Result<usize, Box<dyn Error>> {
    use schema::users::dsl::*;
    let conn = establish_connection()?;

    let rows = diesel::insert_into(users)
        .values(user)
        .execute(&conn);

    Ok(rows.unwrap())
}

pub async fn add_user_auth(user: &AuthUser) -> Result<usize, Box<dyn Error>> {
    use schema::users::dsl::*;
    let conn = establish_connection()?;

    let rows = diesel::insert_into(users)
        .values(user)
        .execute(&conn);

    Ok(rows.unwrap())
}

pub async fn delete_user(user_id: i32) -> Result<usize, Box<dyn Error>> {
    use schema::users::dsl::*;
    let conn = establish_connection()?;

    let rows = diesel::delete(users.find(user_id))
        .execute(&conn);

    Ok(rows.unwrap())
}

pub async fn delete_user_auth(auth_id: &String) -> Result<usize, Box<dyn Error>> {
    use schema::users::dsl::*;
    let conn = establish_connection()?;

    let rows = diesel::delete(
        users.filter(auth0_id.eq(auth_id)))
        .execute(&conn);

    Ok(rows.unwrap())
}