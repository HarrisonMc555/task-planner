extern crate diesel;
extern crate task_planner_server;

use self::task_planner_server::*;
use std::io::stdin;

fn main() -> Result<(), diesel::result::Error> {
    let connection = establish_connection();

    println!("Enter your name");
    let mut display_name = String::new();
    stdin().read_line(&mut display_name).unwrap();
    let display_name = display_name.trim_end();

    println!("Choose a username");
    let mut username_buffer = String::new();
    let user;
    loop {
        username_buffer.clear();
        stdin().read_line(&mut username_buffer).unwrap();
        let username = username_buffer.trim_end();

        match create_user(&connection, username, display_name) {
            Ok(u) => {
                user = u;
                break;
            }

            Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            )) => println!("This username is already taken. Please choose another username."),

            Err(e) => return Err(e),
        }
    }

    println!("Created user with id: {}", user.id);

    Ok(())
}
