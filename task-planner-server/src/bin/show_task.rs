extern crate diesel;
extern crate task_planner_server;

use self::task_planner_server::connection::*;
use std::env::args;

fn main() {
    let connection = establish_connection();

    let task_id = args()
        .nth(1)
        .expect("Task ID argument required")
        .parse::<i32>()
        .expect("Task ID must be  valid integer");

    match task_planner_server::tasks::get(&connection, task_id) {
        Ok(task) => println!("{:?}", task),
        Err(diesel::result::Error::NotFound) => println!("No task with ID {} found", task_id),
        Err(e) => println!("Internal database error: {:?}", e),
    }
}
