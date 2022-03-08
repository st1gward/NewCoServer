#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket::{get, routes};
use rocket::{Request, Response};
use serde::{Serialize, Deserialize};
use rocket_contrib::json::Json;

mod models;
use crate::models::user::User;

#[derive(Serialize)]
struct StatusMessage {
    message: String,
}


/*
#[post("/api/profile", format = "text/plain", data = "<data>")]
fn update_profile(data: String) -> &'static str {
    let user: User = serde_json::from_str(&data).unwrap();
    //println!("{:?}", &user);
    
    "POST Complete"
}
*/

#[get("/", format = "text/html")]
fn index() -> &'static str {
    "Hello World"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
