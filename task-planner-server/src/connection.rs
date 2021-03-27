#![allow(unused_imports)]

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::dotenv;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::env;
use std::ops::Deref;

// type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> PgPooledConnection {
    establish_pool().get().expect("Error getting connection")
}

pub fn establish_pool() -> PgPool {
    dotenv().ok();

    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    init_pool(&database_url()).expect("Failed to create pool")
}

fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

// pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

// impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
//     type Error = ();

//     fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
//         let pool = request.guard::<State<Pool>>()?;
//         match pool.get() {
//             Ok(conn) => Outcome::Success(DbConn(conn)),
//             Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
//         }
//     }
// }

// impl Deref for DbConn {
//     type Target = PgConnection;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
