#![feature(proc_macro_hygiene, decl_macro)]
extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate task_planner_server;

pub fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
