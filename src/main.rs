#[macro_use] extern crate rocket;

use rocket::{Build, Rocket, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello Rocket !!!!"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}