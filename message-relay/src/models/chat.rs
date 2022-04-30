use super::*;

#[derive(Queryable)]
pub struct Chat {
    pub id: i32,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}