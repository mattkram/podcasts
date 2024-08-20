#[macro_use]
extern crate rocket;

use rocket::fairing::{self, AdHoc};
use rocket::fs::FileServer;
use rocket::{Build, Rocket};
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

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Logs::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("./migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Logs::init())
        .attach(AdHoc::try_on_ignite("Run SQLx Migrations", run_migrations))
        .mount("/", routes![index, handle_click, read])
        .mount("/static", FileServer::from("./static"))
        .attach(Template::fairing())
}
