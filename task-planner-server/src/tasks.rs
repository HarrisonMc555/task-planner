use diesel::prelude::*;

use crate::models::{NewTask, Task, User};
use crate::schema::tasks::{self, dsl};

pub fn for_user(conn: &PgConnection, user_id: i32) -> QueryResult<Vec<Task>> {
    let user = crate::users::get(conn, user_id)?;
    Task::belonging_to(&user).load(conn)
}

pub fn by_id(conn: &PgConnection, id: i32) -> QueryResult<Task> {
    tasks::table.find(id).get_result::<Task>(conn)
}

pub fn create_task(conn: &PgConnection, task: &NewTask) -> QueryResult<Task> {
    diesel::insert_into(tasks::table)
        .values(task)
        .get_result(conn)
}

pub fn complete_task(conn: &PgConnection, id: i32) -> QueryResult<Task> {
    set_task_complete(conn, id, true)
}

pub fn incomplete_task(conn: &PgConnection, id: i32) -> QueryResult<Task> {
    set_task_complete(conn, id, false)
}

pub fn set_task_complete(conn: &PgConnection, id: i32, is_complete: bool) -> QueryResult<Task> {
    diesel::update(dsl::tasks.find(id))
        .set(dsl::complete.eq(is_complete))
        .get_result::<Task>(conn)
}

pub fn incomplete_for_user(conn: &PgConnection, user: &User) -> QueryResult<Vec<Task>> {
    Task::belonging_to(user)
        .filter(dsl::complete.eq(false))
        .load::<Task>(conn)
}
