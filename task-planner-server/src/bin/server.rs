#![feature(proc_macro_hygiene, decl_macro)]
extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate task_planner_server;

use diesel::QueryResult;
use rocket_contrib::json::Json;

use task_planner_server::models::{NewTask, Task, User};
use task_planner_server::{connection, tasks, users};

pub fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![index, get_user_tasks, get_users, get_user, create_task],
        )
        .launch();
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/user/<username>/tasks")]
fn get_user_tasks(username: String) -> QueryResult<Option<Json<Vec<Task>>>> {
    let connection = connection::establish_connection();
    let user = match users::user_by_username(&connection, &username)? {
        Some(user) => user,
        None => return Ok(None),
    };
    tasks::all(&connection, user.id).map(|tasks| Some(Json(tasks)))
}

#[get("/users")]
fn get_users() -> QueryResult<Json<Vec<User>>> {
    let connection = connection::establish_connection();
    users::all_users(&connection).map(|users| Json(users))
}

#[get("/user/<username>")]
fn get_user(username: String) -> QueryResult<Option<Json<User>>> {
    let connection = connection::establish_connection();
    users::user_by_username(&connection, &username)
        .map(|option_user| option_user.map(|user| Json(user)))
}

#[post("/task/new", data = "<task>")]
fn create_task(task: Json<NewTask>) -> QueryResult<Json<Task>> {
    let connection = connection::establish_connection();
    tasks::create_task(&connection, &task).map(|new_task| Json(new_task))
}
