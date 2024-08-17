use rocket::{get, launch, routes};
// Alternate which will expose all macros globally
// See note in: https://rocket.rs/guide/v0.5/overview/#routing
// #[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
