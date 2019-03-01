use crate::models::user::User;
use crate::schema::sites;
use chrono::NaiveDateTime;

#[derive(Deserialize, Serialize, Identifiable, Queryable, Associations)]
#[belongs_to(User)]
pub struct Site {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "sites"]
pub struct NewSite {
    name: String,
    user_id: i32,
}
