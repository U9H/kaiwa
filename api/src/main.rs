#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

mod schema;

use rocket::request::Request;
use rocket_contrib::databases::diesel::PgConnection;

mod controllers;
mod models;
use crate::controllers as c;

#[catch(503)]
fn service_not_available(_req: &Request) -> &'static str {
    "Service is not available. (Is the database up?)"
}

#[get("/")]
fn index() -> &'static str {
    "Kaiwa is running."
}

#[database("kaiwa-db")]
pub struct Conn(PgConnection);

fn main() {
    rocket::ignite()
        .attach(Conn::fairing())
        .register(catchers![service_not_available])
        .mount("/api/v1", routes![c::sites::create, c::users::create])
        .mount("/", routes![index])
        .launch();
}
