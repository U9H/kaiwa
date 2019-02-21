use crate::Conn;

#[post("/sites")]
pub fn create(_conn: Conn) -> String {
    // Rocket uses the Conn request guard to provide us with a database
    // connection from a managed pool.
    String::from("Hello, from Rust! (with a database connection!)")
}

// #[get("/sites/<name>/<age>/<cool>")]
