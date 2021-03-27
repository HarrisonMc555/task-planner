extern crate diesel;
extern crate task_planner_server;

use self::task_planner_server::connection::*;
use std::io::stdin;

fn main() {
    let connection = establish_connection();

    println!("Enter task ID");
    let mut task_id_buffer = String::new();
    stdin().read_line(&mut task_id_buffer).unwrap();
    let task_id = task_id_buffer.trim_end();
    let task_id = task_id.parse::<i32>().expect("Invalid task ID");

    match task_planner_server::tasks::get(&connection, task_id) {
        Ok(task) => println!("{:?}", task),
        Err(diesel::result::Error::NotFound) => println!("No task with ID {} found", task_id),
        Err(e) => println!("Internal database error: {:?}", e),
    }
}
