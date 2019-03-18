use crate::schema::users::dsl::*;
use crate::Conn;
use crate::models::user::*;
use diesel::prelude::*;
use rocket_contrib::json::Json;

#[post("/users", format = "application/json", data = "<user>")]
pub fn create(c: Conn, user: Json<NewUser>) -> Result<Json<User>, diesel::result::Error> {
    Ok(Json(
        diesel::insert_into(users)
            .values(&user.into_inner())
            .get_result(&*c)?,
    ))
}

// #[get("/sites/<name>/<age>/<cool>")]
