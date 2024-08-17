use rocket::{get, launch, routes};
// Alternate which will expose all macros globally
// See note in: https://rocket.rs/guide/v0.5/overview/#routing
// #[macro_use] extern crate rocket;
use rocket::fs::FileServer;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello])
        .mount("/static", FileServer::from("./static"))
}
