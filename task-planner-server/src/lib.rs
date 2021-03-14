pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use self::models::{NewUser, User};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn user_by_username(conn: &PgConnection, username: &str) -> Option<User> {
    use schema::users::dsl::{username as dsl_username, users};
    users
        .limit(1)
        .filter(dsl_username.eq(username))
        .load::<User>(conn)
        .ok()
        .and_then(|u| u.into_iter().next())
}

pub fn create_user<'a>(
    conn: &PgConnection,
    username: &'a str,
    display_name: &'a str,
) -> Result<User, diesel::result::Error> {
    use schema::users;

    let new_user = NewUser {
        username,
        display_name,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}
