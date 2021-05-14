#![feature(proc_macro_hygiene, decl_macro)]
extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate task_planner_server;

use std::collections::HashMap;

use diesel::QueryResult;
use rocket::response::Redirect;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use serde_json::json;

use task_planner_server::models::{NewTask, Task, User};
use task_planner_server::{connection, tasks, users};

pub fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                get_user_tasks,
                complete_task,
                incomplete_task,
                get_users,
                get_user,
                create_task
            ],
        )
        .attach(Template::fairing())
        .launch();
}

#[get("/")]
fn index() -> Template {
    Template::render("index", &HashMap::<(), ()>::new())
}

#[get("/user/<username>/tasks")]
fn get_user_tasks(username: String) -> Option<Template> {
    let connection = connection::establish_connection();
    let user = users::user_by_username(&connection, &username).ok()??;
    let mut tasks = tasks::all(&connection, user.id).ok()?;
    tasks.sort_unstable_by_key(|t| t.id);
    let mut context = HashMap::new();
    context.insert("user", json!(user));
    context.insert("tasks", json!(tasks));
    Some(Template::render("tasks", &context))
}

#[get("/user/<username>/complete/<task_id>")]
fn complete_task(username: String, task_id: i32) -> Option<Redirect> {
    let connection = connection::establish_connection();
    let user = users::user_by_username(&connection, &username).ok()??;
    let task = tasks::get(&connection, task_id).ok()?;
    if task.user_id != user.id {
        return None;
    }
    tasks::complete_task(&connection, task_id).ok()?;
    Some(Redirect::to(uri!(get_user_tasks: username)))
}

#[get("/user/<username>/incomplete/<task_id>")]
fn incomplete_task(username: String, task_id: i32) -> Option<Redirect> {
    let connection = connection::establish_connection();
    let user = users::user_by_username(&connection, &username).ok()??;
    let task = tasks::get(&connection, task_id).ok()?;
    if task.user_id != user.id {
        return None;
    }
    tasks::incomplete_task(&connection, task_id).ok()?;
    Some(Redirect::to(uri!(get_user_tasks: username)))
}

#[get("/users")]
fn get_users() -> QueryResult<Json<Vec<User>>> {
    let connection = connection::establish_connection();
    users::all_users(&connection).map(Json)
}

#[get("/user/<username>")]
fn get_user(username: String) -> QueryResult<Option<Json<User>>> {
    let connection = connection::establish_connection();
    users::user_by_username(&connection, &username).map(|option_user| option_user.map(Json))
}

#[post("/task/new", data = "<task>")]
fn create_task(task: Json<NewTask>) -> QueryResult<Json<Task>> {
    let connection = connection::establish_connection();
    tasks::create_task(&connection, &task).map(Json)
}
