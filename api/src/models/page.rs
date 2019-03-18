use crate::models::site::Site;
use crate::schema::pages;
use chrono::NaiveDateTime;

#[derive(Deserialize, Serialize, Identifiable, Queryable, Associations)]
#[belongs_to(Site)]
pub struct Page {
    pub id: i32,
    pub site_id: i32,
    pub location: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "pages"]
pub struct NewPage {
    site_id: i32,
    location: String,
}
