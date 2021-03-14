extern crate diesel;
extern crate task_planner_server;

use self::task_planner_server::*;
use std::io::stdin;

fn main() {
    let connection = establish_connection();

    println!("Enter your name");
    let mut display_name = String::new();
    stdin().read_line(&mut display_name).unwrap();
    let display_name = display_name.trim_end();

    println!("Choose a username");
    let mut username_buffer = String::new();
    let username;
    loop {
        stdin().read_line(&mut username_buffer).unwrap();
        let username_try = username_buffer.trim_end();
        if user_by_username(&connection, username_try).is_none() {
            username = username_try;
            break;
        }
        println!("This username is already taken. Please choose another username.");
        username_buffer.clear();
    }

    let user = match create_user(&connection, username, display_name) {
        Ok(user) => user,
        Err(e) => {
            println!("Error adding user: {:?}", e);
            return;
        }
    };
    println!("Created user with id: {}", user.id);
}
