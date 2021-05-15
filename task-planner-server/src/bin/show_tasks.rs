extern crate diesel;
extern crate task_planner_server;

use diesel::QueryResult;

use task_planner_server::helper;

use self::task_planner_server::connection::*;

fn main() -> QueryResult<()> {
    let connection = establish_connection();

    let user = helper::get_username_by_username_from_stdin(&connection)?;

    let results =
        task_planner_server::tasks::for_user(&connection, user.id).expect("Error loading tasks");

    println!("Displaying {} tasks", results.len());
    for task in results {
        println!("{:?}", task);
    }

    Ok(())
}
