use super::*;

#[derive(Queryable)]
pub struct UserChat {
    pub id: i32,
    pub user_id: i32,
    pub chat_id: i32,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}