use std::io::stdin;

use diesel::QueryResult;

use crate::models::User;
use crate::{connection::*, users::*};

pub fn get_username_by_username_from_stdin(connection: &PgPooledConnection) -> QueryResult<User> {
    println!("Enter your username");
    let mut username_buffer = String::new();
    loop {
        username_buffer.clear();
        stdin().read_line(&mut username_buffer).unwrap();
        let username = username_buffer.trim_end();

        match user_by_username(connection, username) {
            Ok(Some(user)) => return Ok(user),

            Ok(None) => println!(
                "No user found with username {}. Please try again.",
                username
            ),

            Err(e) => return Err(e),
        }
    }
}
