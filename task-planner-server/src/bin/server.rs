#![allow(unused_imports)]
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

extern crate diesel;
extern crate task_planner_server;

use self::task_planner_server::*;
use std::env::args;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use std::env;
// use rocket_contrib::Json;

pub fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
