#![feature(proc_macro_hygiene, decl_macro)]
extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate task_planner_server;

use rocket_contrib::json::Json;

use diesel::QueryResult;
use task_planner_server::models::User;
use task_planner_server::{connection, tasks, users};

pub fn main() {
    rocket::ignite()
        .mount("/", routes![index, get_user_tasks, get_users])
        .launch();
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/user/<user_id>/tasks")]
fn get_user_tasks(user_id: i32) -> String {
    let connection = connection::establish_connection();
    let tasks = match tasks::all(&connection, user_id) {
        Ok(tasks) => tasks,
        Err(e) => return format!("{:?}", e),
    };
    if tasks.is_empty() {
        return "No tasks".to_string();
    }
    tasks
        .into_iter()
        .map(|task| {
            let checkbox = if task.complete { "X" } else { " " };
            format!("[{}] {}\n", checkbox, task.title)
        })
        .collect()
}

#[get("/users")]
fn get_users() -> QueryResult<Json<Vec<User>>> {
    let connection = connection::establish_connection();
    users::all_users(&connection).map(|users| Json(users))
}
