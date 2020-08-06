use diesel::Connection;
use diesel::MysqlConnection;
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn new_conn(dsn: &str) -> Result<MysqlConnection> {
    Ok(MysqlConnection::establish(&dsn)?)
}
