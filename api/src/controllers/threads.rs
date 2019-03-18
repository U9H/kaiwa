use crate::models::thread::{NewThread, Thread};
use crate::schema::threads::dsl::*;
use crate::Conn;
use diesel::prelude::*;
use rocket_contrib::json::Json;

#[post("/sites/<num_site_id>/pages/<num_page_id>/threads", format = "application/json", data = "<thread>")]
pub fn create(c: Conn, thread: Json<NewThread>, num_site_id: i32, num_page_id: i32 ) -> Result<Json<Thread>, diesel::result::Error> {
    Ok(Json(
        diesel::insert_into(threads)
            .values(
                &thread.into_inner()
            )
            .get_result(&*c)?,
    ))
}
