#![feature(decl_macro, proc_macro_hygiene)]

pub mod dao;
pub mod handler;
pub mod model;
pub mod schema;
pub mod result;

extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenv::dotenv;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use std::env;
use std::ops::Deref;
use std::time;

pub struct MysqlConn(PooledConnection<ConnectionManager<MysqlConnection>>);

impl Deref for MysqlConn {
    type Target = MysqlConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for MysqlConn {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let pool = request.guard::<State<Pool<ConnectionManager<MysqlConnection>>>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(MysqlConn(conn)),
            Err(_) => rocket::Outcome::Failure((Status::InternalServerError, ())),
        }
    }
}

fn main() {
    dotenv().ok().unwrap();
    let url = env::var("DATABASE_URL").expect("no database url");
    let pool = Pool::builder()
        .max_size(10)
        .max_lifetime(Some(time::Duration::from_secs(300)))
        .build(ConnectionManager::<MysqlConnection>::new(url))
        .expect("failed to construct connection pool");
    rocket::ignite()
        .manage(pool)
        .mount(
            "/",
            routes![
                handler::add_device_info,
                handler::bulk_add_device_info,
                handler::query_device_info,
                handler::get_device_info,
                handler::delete_device_info,
                handler::update_device_info,
                handler::add_device,
                handler::bulk_add_device,
                handler::get_device,
                handler::query_device,
                handler::update_device,
                handler::delete_device,
                handler::query_subsystem,
                
            ],
        )
        .launch();
}
