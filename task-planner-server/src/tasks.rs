use crate::models::Task;
use crate::schema::tasks::dsl;
use diesel;
use diesel::prelude::*;

pub fn all(conn: &PgConnection, user_id: i32) -> QueryResult<Vec<Task>> {
    dsl::tasks
        .filter(dsl::user_id.eq(user_id))
        .load::<Task>(conn)
}
