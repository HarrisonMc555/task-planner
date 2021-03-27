extern crate diesel;
extern crate task_planner_server;

use self::task_planner_server::connection::*;
use std::env::args;

fn main() {
    let connection = establish_connection();

    let user_id = args()
        .nth(1)
        .expect("User ID argument required")
        .parse::<i32>()
        .expect("User ID must be  valid integer");

    let results = task_planner_server::tasks::all(&connection, user_id).expect("Error loading tasks");

    println!("Displaying {} tasks", results.len());
    for task in results {
        println!("{:?}", task);
    }
}
