use crate::models::{NewUser, User};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn user_by_username(conn: &PgConnection, username: &str) -> QueryResult<Option<User>> {
    use crate::schema::users::dsl::{username as dsl_username, users};
    users
        .limit(1)
        .filter(dsl_username.eq(username))
        .load::<User>(conn)
        .map(|u| u.into_iter().next())
}

pub fn username_taken(conn: &PgConnection, username: &str) -> Result<bool, diesel::result::Error> {
    use crate::schema::users::dsl::{username as dsl_username, users};
    use diesel::dsl::{exists, select};
    select(exists(users.filter(dsl_username.eq(username)))).get_result(conn)
}

pub fn username_available(
    conn: &PgConnection,
    username: &str,
) -> Result<bool, diesel::result::Error> {
    let taken = username_taken(conn, username)?;
    Ok(!taken)
}

pub fn create_user<'a>(
    conn: &PgConnection,
    username: &'a str,
    display_name: &'a str,
) -> Result<User, diesel::result::Error> {
    use crate::schema::users;

    let new_user = NewUser {
        username,
        display_name,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}
