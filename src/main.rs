#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
pub mod models;

use std::collections::HashMap;
use self::models::user::User;

#[get("/")]
fn index() -> &'static str {
    "Welcome to My Blog"
}

#[get("/about")]
fn about() -> String {
    let mut data = HashMap::new();
    data.insert("id", "2");
    data.insert("first_name", "Timothy");
    data.insert("last_name", "Onyiuke");
    data.insert("username", "timolinn");
    data.insert("password", "jargon_passowrd");

    let tim = User::new(data);
    tim.first_name
}


fn main() {
    rocket::ignite().mount("/", routes![index, about]).launch();
}
