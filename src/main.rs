#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
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
        .attach(Template::fairing())
}
