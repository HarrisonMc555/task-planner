extern crate diesel;
extern crate task_planner_server;

use self::task_planner_server::{connection::*, helper, models::NewTask, tasks::*};
use std::io::stdin;

fn main() -> Result<(), diesel::result::Error> {
    let connection = establish_connection();

    let user = helper::get_username_by_username_from_stdin(&connection)?;

    println!("Enter task title:");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();

    let task = NewTask {
        user_id: user.id,
        title,
        description: None,
        due: None,
    };

    match create_task(&connection, &task) {
        Ok(task) => println!("Created task with id: {}", task.id),
        Err(e) => println!("Error creating task: {:?}", e),
    }

    Ok(())
}
