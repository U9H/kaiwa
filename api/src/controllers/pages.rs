use crate::models::page::{NewPage, Page};
use crate::schema::pages::dsl::*;
use crate::Conn;
use diesel::prelude::*;
use rocket_contrib::json::Json;

// We use "num_site_id" as to not conflict with the "site_id" imported via the glob
// In the future, it will probably be best to reference the site by domain/url in addition to ID
// TODO: Note thate in both pages and threads, the data is still gathered from the JSON object.
// We need to extract this from the URL instead.
#[post("/sites/<num_site_id>/pages", format = "application/json", data = "<page>")]
pub fn create(c: Conn, page: Json<NewPage>, num_site_id: i32 ) -> Result<Json<Page>, diesel::result::Error> {
    Ok(Json(
        diesel::insert_into(pages)
            .values(
                &page.into_inner()
            )
            .get_result(&*c)?,
    ))
}
