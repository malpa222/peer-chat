use super::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Chat {
    pub id: i32,
    #[serde(skip)]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(skip)]
    pub updated_at: Option<chrono::NaiveDateTime>,
}