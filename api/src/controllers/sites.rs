use crate::models::site::{NewSite, Site};
use crate::schema::sites::dsl::*;
use crate::Conn;
use diesel::prelude::*;
use rocket_contrib::json::Json;

#[post("/sites", format = "application/json", data = "<site>")]
pub fn create(c: Conn, site: Json<NewSite>) -> Result<Json<Site>, diesel::result::Error> {
    Ok(Json(
        diesel::insert_into(sites)
            .values(&site.into_inner())
            .get_result(&*c)?,
    ))
}

// #[get("/sites/<name>/<age>/<cool>")]
