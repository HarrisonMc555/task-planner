extern crate diesel;
extern crate task_planner_server;

use self::task_planner_server::{tasks::*, users::*, connection::*};
use std::io::stdin;

fn main() -> Result<(), diesel::result::Error> {
    let connection = establish_connection();

    println!("Enter your username");
    let mut username_buffer = String::new();
    let user;
    loop {
        username_buffer.clear();
        stdin().read_line(&mut username_buffer).unwrap();
        let username = username_buffer.trim_end();

        match user_by_username(&connection, username) {
            Ok(Some(u)) => {
                user = u;
                break;
            }

            Ok(None) => println!(
                "No user found with username {}. Please try again.",
                username
            ),

            Err(e) => return Err(e),
        }
    }

    let tasks = task_planner_server::tasks::get_incomplete(&connection, user.id).expect("Error loading tasks");
    if tasks.is_empty() {
        println!("No incomplete tasks");
        return Ok(());
    }

    println!("Displaying {} tasks", tasks.len());
    for task in tasks {
        println!("#{} {}", task.id, task.title);
    }

    println!("Which task to complete?");
    let mut task_id_buffer = String::new();
    stdin().read_line(&mut task_id_buffer).unwrap();
    let task_id = task_id_buffer.trim_end();
    let task_id = task_id.parse::<i32>().expect("Invalid task ID");

    match complete_task(&connection, task_id) {
        Ok(task) => println!("Completed task: {:?}", task),
        Err(e) => println!("Error completing task: {:?}", e),
    }

    Ok(())
}
