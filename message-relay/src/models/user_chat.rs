use super::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct UserChat {
    pub id: i32,
    pub user_id: i32,
    pub chat_id: i32,
    #[serde(skip)]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(skip)]
    pub updated_at: Option<chrono::NaiveDateTime>,
}