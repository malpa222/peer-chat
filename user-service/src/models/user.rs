use super::*;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct User {
    #[serde(skip)]
    pub id: i32,
    pub auth0_id: Option<String>,
    pub email: String,
    pub username: String,
    #[serde(skip)]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(skip)]
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct ApiUser {
    pub email: String,
    pub username: String,
}

#[derive(Insertable, Deserialize, Clone)]
#[table_name="users"]
pub struct AuthUser {
    pub auth0_id: String,
    pub email: String,
    pub username: String
}
