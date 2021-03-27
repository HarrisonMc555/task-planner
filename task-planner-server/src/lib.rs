pub mod connection;
pub mod models;
pub mod schema;
pub mod tasks;
pub mod users;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
