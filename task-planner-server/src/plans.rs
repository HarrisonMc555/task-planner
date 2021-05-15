use diesel::prelude::*;

use crate::models::{NewPlan, Plan, Task, TaskPlans, User};
use crate::schema::plans::dsl as plans_dsl;
use crate::schema::tasks::dsl as tasks_dsl;

pub fn task_plans_for_user(conn: &PgConnection, user: &User) -> QueryResult<Vec<TaskPlans>> {
    let tasks = Task::belonging_to(user).load::<Task>(conn)?;
    tasks
        .into_iter()
        .map(|task| {
            let plans = for_task(conn, &task)?;
            Ok(TaskPlans { task, plans })
        })
        .collect()
}

pub fn for_task(conn: &PgConnection, task: &Task) -> QueryResult<Vec<Plan>> {
    Plan::belonging_to(task).load(conn)
}

pub fn by_id(conn: &PgConnection, id: i32) -> QueryResult<Plan> {
    plans_dsl::plans.find(id).get_result::<Plan>(conn)
}

pub fn create_plan(conn: &PgConnection, plan: &NewPlan) -> QueryResult<Plan> {
    diesel::insert_into(plans_dsl::plans)
        .values(plan)
        .get_result(conn)
}

pub fn complete_plan(conn: &PgConnection, id: i32) -> QueryResult<Plan> {
    set_plan_complete(conn, id, true)
}

pub fn incomplete_plan(conn: &PgConnection, id: i32) -> QueryResult<Plan> {
    set_plan_complete(conn, id, false)
}

pub fn set_plan_complete(conn: &PgConnection, id: i32, is_complete: bool) -> QueryResult<Plan> {
    diesel::update(plans_dsl::plans.find(id))
        .set(plans_dsl::complete.eq(is_complete))
        .get_result::<Plan>(conn)
}

pub fn get_incomplete(conn: &PgConnection, task_id: i32) -> QueryResult<Vec<Plan>> {
    let task = tasks_dsl::tasks.find(task_id).get_result::<Task>(conn)?;
    Plan::belonging_to(&task)
        .filter(plans_dsl::complete.eq(false))
        .load::<Plan>(conn)
}
