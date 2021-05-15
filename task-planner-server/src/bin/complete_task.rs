extern crate diesel;
extern crate task_planner_server;

use std::io::stdin;

use self::task_planner_server::{connection::*, helper, tasks::*};

fn main() -> Result<(), diesel::result::Error> {
    let connection = establish_connection();

    let user = helper::get_username_by_username_from_stdin(&connection)?;

    let tasks = task_planner_server::tasks::incomplete_for_user(&connection, &user)
        .expect("Error loading tasks");
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
