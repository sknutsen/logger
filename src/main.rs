#[macro_use] 
extern crate rocket;
extern crate chrono;

mod controllers;
mod handlers;
mod models;

use controllers::log::create_log;
use models::{log_entry::LogEntry, log_request::LogRequest};

use rocket::serde::json::Json;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/log", format = "application/json", data = "<req>")]
fn log(req: Json<LogRequest>) {
    let new_entry: LogEntry = create_log(req.text.to_string(), req.source.to_string());

    println!("time: {} ---text: {} ---source: {}", new_entry.time.to_string(), new_entry.text, new_entry.source);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, log])
}
