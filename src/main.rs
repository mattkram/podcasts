// Alternate which will expose all macros globally
// See note in: https://rocket.rs/guide/v0.5/overview/#routing
// #[macro_use] extern crate rocket;
use rocket::fs::FileServer;
use rocket::{get, launch, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/clicked")]
fn handle_click() -> &'static str {
    "<h3>Ouch!</h3>"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello, handle_click])
        .mount("/static", FileServer::from("./static"))
}
