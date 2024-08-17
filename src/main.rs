#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, NamedFile};
use std::path::Path;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("templates/index.html"))
        .await
        .ok()
}

#[get("/clicked")]
fn handle_click() -> &'static str {
    "<h3>Ouch!</h3>"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, handle_click])
        .mount("/static", FileServer::from("./static"))
}
