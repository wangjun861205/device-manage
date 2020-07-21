use rocket::http::Header;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket_contrib::json::Json;
use std::fmt;
use std::io::Cursor;
use std::convert::From;
use diesel::result::Error;

#[derive(PartialEq, Debug)]
pub struct DaoError {
    detail: String,
}

impl DaoError {
    pub fn new(detail: &str) -> Self {
        DaoError {
            detail: detail.to_owned(),
        }
    }
}

impl fmt::Display for DaoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DaoError: {}", self.detail)
    }
}

impl From<Error> for DaoError {
    fn from(e: Error) -> Self {
        Self::new(&format!("{}", e))
    }
}


impl<'r> Responder<'r> for DaoError {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(format!("数据库错误: {}", self.detail)))
            .status(Status::InternalServerError)
            .header(Header::new("Content-Type", "text/plain;charset=utf-8"))
            .ok()
    }
}

pub type Result<T> = std::result::Result<Json<T>, DaoError>;
