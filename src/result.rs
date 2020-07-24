use rocket::response::Responder;
use rocket::Request;
use rocket::response;
use std::io::Cursor;
use rocket::http::{ Status, Header };
use std::fmt;
use std::fmt::Display;
use diesel;
use rocket_contrib::json::Json;

#[derive(Debug)]
pub struct Error {
    status: Status,
    detail: String,
}

impl Error {
    pub fn new(status: Status, detail: String) -> Self {
        Self {
            status,
            detail,
        }
    }
}

impl Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.status, self.detail)
        }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Error {
        Self::new(Status::new(599, "database error"), format!("{}", e))
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        response::ResponseBuilder::new(response::Response::new())
        .status(self.status)
        .sized_body(Cursor::new(self.detail))
        .header(Header::new("Content-Type", "text/plain;charset=utf-8"))
        .ok()
    }
}


pub type Result<T> = std::result::Result<Json<T>, Error>;