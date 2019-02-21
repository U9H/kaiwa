use crate::models::user::User;
use chrono::NaiveDateTime;

use crate::schema::posts;

#[derive(Queryable, Associations, Identifiable)]
#[belongs_to(User)]
pub struct Site {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}