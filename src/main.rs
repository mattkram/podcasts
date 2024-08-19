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

#[get("/clicked?<name>")]
fn handle_click(name: Option<&str>) -> String {
    let mut _name = "Dude";

    match name {
        Some(n) => _name = n,
        _ => (),
    }
    format!("Ouch, {}!", _name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, handle_click])
        .mount("/static", FileServer::from("./static"))
}
