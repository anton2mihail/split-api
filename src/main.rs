#[macro_use] extern crate rocket;
// use rocket::http::Status;
// use rocket::response::{content, status};
// use rocket::serde::json::Json;

mod seed;

#[get("/")]
fn index() -> &'static str {
    "ok!"
}


#[get("/catalogue")]
fn catalogue() -> &'static str {
    "ok!"
}

#[launch]
fn rocket() -> _ {
    seed::seed_db();
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![catalogue])
}
