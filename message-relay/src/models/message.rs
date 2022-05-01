use super::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub user_id: i32,
    pub chat_id: i32,
    pub content: String,
    #[serde(skip)]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(skip)]
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="messages"]
pub struct ApiMessage {
    pub user_id: i32,
    pub chat_id: i32,
    pub content: String,
}