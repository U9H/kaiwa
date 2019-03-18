use crate::models::page::Page;
use crate::schema::threads;
use chrono::NaiveDateTime;

#[derive(Deserialize, Serialize, Identifiable, Queryable, Associations)]
#[belongs_to(Page)]
pub struct Thread {
    pub id: i32,
    pub page_id: i32,
    /// This is in the case that this is a reply to another thread
    pub thread_id: Option<i32>,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "threads"]
pub struct NewThread {
    page_id: i32,
    thread_id: Option<i32>,
    body: String,
}
