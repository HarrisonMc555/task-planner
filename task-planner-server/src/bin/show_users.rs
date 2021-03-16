extern crate diesel;
extern crate task_planner_server;

use self::diesel::prelude::*;
use self::models::*;
use self::task_planner_server::*;

fn main() {
    use task_planner_server::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(50)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!(
            "Username: '{}', display name: '{}'",
            user.username, user.display_name
        );
    }
}
