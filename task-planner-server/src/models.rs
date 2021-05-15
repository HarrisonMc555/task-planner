use super::schema::{plans, tasks, users};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub display_name: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub display_name: &'a str,
}

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Debug)]
#[belongs_to(User)]
pub struct Task {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub due: Option<NaiveDateTime>,
    pub complete: bool,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "tasks"]
pub struct NewTask<'a> {
    pub user_id: i32,
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub due: Option<NaiveDateTime>,
}

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Debug)]
#[belongs_to(Task)]
pub struct Plan {
    pub id: i32,
    pub task_id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub due: NaiveDateTime,
    pub complete: bool,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "plans"]
pub struct NewPlan<'a> {
    pub task_id: i32,
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub due: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskPlans {
    pub task: Task,
    pub plans: Vec<Plan>,
}
