use super::schema::{tasks, users};
use crate::util::NaiveDateTimeForm;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
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

#[derive(Queryable, Serialize, Deserialize, Debug)]
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

#[derive(FromForm, Debug)]
pub struct FormTask {
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub due: Option<NaiveDateTimeForm>,
}

impl FormTask {
    pub fn create_new_task(&self) -> NewTask {
        NewTask {
            user_id: self.user_id,
            title: &self.title,
            description: self.description.as_deref(),
            due: self.due.as_deref().cloned(),
        }
    }
}
