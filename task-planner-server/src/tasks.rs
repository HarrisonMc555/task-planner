use crate::models::{NewTask, Task};
use crate::schema::tasks::{self, dsl};
use diesel;
use diesel::prelude::*;

pub fn all(conn: &PgConnection, user_id: i32) -> QueryResult<Vec<Task>> {
    dsl::tasks
        .filter(dsl::user_id.eq(user_id))
        .load::<Task>(conn)
}

pub fn get(conn: &PgConnection, id: i32) -> QueryResult<Task> {
    tasks::table.find(id).get_result::<Task>(conn)
}

pub fn create_task<'a>(conn: &PgConnection, task: &NewTask) -> Result<Task, diesel::result::Error> {
    diesel::insert_into(tasks::table)
        .values(task)
        .get_result(conn)
}

pub fn complete_task(conn: &PgConnection, id: i32) -> QueryResult<Task> {
    diesel::update(dsl::tasks.find(id))
        .set(dsl::complete.eq(true))
        .get_result::<Task>(conn)
}

pub fn get_incomplete(conn: &PgConnection, user_id: i32) -> QueryResult<Vec<Task>> {
    dsl::tasks
        .filter(dsl::user_id.eq(user_id))
        .filter(dsl::complete.eq(false))
        .load::<Task>(conn)
}
