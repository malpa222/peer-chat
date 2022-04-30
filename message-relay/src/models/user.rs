use super::*;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}