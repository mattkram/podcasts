#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

#[derive(Database)]
#[database("sqlite_logs")]
struct Logs(sqlx::SqlitePool);

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/<id>")]
async fn read(mut db: Connection<Logs>, id: i64) -> Option<String> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS logs (id INTEGER PRIMARY KEY AUTOINCREMENT, content TEXT);",
    )
    .execute(&mut **db)
    .await
    .ok();

    sqlx::query("SELECT content FROM logs WHERE id = ?")
        .bind(id)
        .fetch_one(&mut **db)
        .await
        .and_then(|r| Ok(r.try_get(0)?))
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
        .attach(Logs::init())
        .mount("/", routes![index, handle_click, read])
        .mount("/static", FileServer::from("./static"))
        .attach(Template::fairing())
}
