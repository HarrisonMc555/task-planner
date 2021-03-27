extern crate diesel;
extern crate task_planner_server;

use self::task_planner_server::{models::NewTask, tasks::*, users::*, connection::*};
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

    println!("Enter task title:");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();

    let task = NewTask {
        user_id: user.id,
        title,
        description: None,
        due: None,
        complete: false,
    };

    match create_task(&connection, &task) {
        Ok(task) => println!("Created task with id: {}", task.id),
        Err(e) => println!("Error creating task: {:?}", e),
    }

    Ok(())
}
