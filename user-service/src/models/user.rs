use super::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    #[serde(skip)]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(skip)]
    pub updated_at: Option<chrono::NaiveDateTime>,
}