use chrono::NaiveDateTime;
use crate::schema::users;

#[derive(Deserialize, Serialize, Identifiable, Queryable, Associations)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    username: String,
    email: String,
}
