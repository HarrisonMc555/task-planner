use crate::models::{NewTask, Task};
use crate::schema::tasks::{self, dsl};
use diesel::prelude::*;

pub fn all(conn: &PgConnection, user_id: i32) -> QueryResult<Vec<Task>> {
    dsl::tasks
        .filter(dsl::user_id.eq(user_id))
        .load::<Task>(conn)
}

pub fn get(conn: &PgConnection, id: i32) -> QueryResult<Task> {
    tasks::table.find(id).get_result::<Task>(conn)
}

pub fn create_task(conn: &PgConnection, task: &NewTask) -> Result<Task, diesel::result::Error> {
    diesel::insert_into(tasks::table)
        .values(task)
        .get_result(conn)
}

pub fn complete_task(conn: &PgConnection, id: i32) -> QueryResult<Task> {
    set_task_complete(conn, id, true)
}

pub fn uncomplete_task(conn: &PgConnection, id: i32) -> QueryResult<Task> {
    set_task_complete(conn, id, false)
}

pub fn set_task_complete(conn: &PgConnection, id: i32, is_complete: bool) -> QueryResult<Task> {
    diesel::update(dsl::tasks.find(id))
        .set(dsl::complete.eq(is_complete))
        .get_result::<Task>(conn)
}

pub fn get_incomplete(conn: &PgConnection, user_id: i32) -> QueryResult<Vec<Task>> {
    dsl::tasks
        .filter(dsl::user_id.eq(user_id))
        .filter(dsl::complete.eq(false))
        .load::<Task>(conn)
}
