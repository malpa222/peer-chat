use super::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    #[serde(skip)]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(skip)]
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct ApiUser {
    pub email: String,
}