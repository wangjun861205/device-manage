use super::dao::*;
use super::model::*;
use diesel;
use diesel::mysql::MysqlConnection;
use std::convert::From;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    ErrBusiness(String),
    ErrInternal(String),
}

impl Error {
    pub fn new_business_error(s: String) -> Self {
        Error::ErrBusiness(s)
    }

    pub fn new_internal_error(s: String) -> Self {
        Error::ErrInternal(s)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::ErrBusiness(s) => write!(f, "{}", s),
            Error::ErrInternal(s) => write!(f, "{}", s),
        }
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Error::ErrInternal(format!("{}", e))
    }
}

type Result<T> = std::result::Result<T, Error>;

pub fn add_subsystem_info_to_device_info(
    conn: &MysqlConnection,
    devinfo_id: i32,
    subinfo_id: i32,
) -> Result<usize> {
    get_device_info(conn, devinfo_id)?;
    get_subsystem_info(conn, subinfo_id)?;
    Ok(insert_deviceinfo_subsysteminfo(
        conn,
        DevinfoSubinfoInsert {
            device_info_id: devinfo_id,
            subsystem_info_id: subinfo_id,
        },
    )?)
}

pub fn remove_subsystem_info_from_device_info(
    conn: &MysqlConnection,
    devinfo_id: i32,
    subinfo_id: i32,
) -> Result<usize> {
    delete_deviceinfo_subsysteminfo(conn, devinfo_id, subinfo_id)?;
    Ok(bulk_delete_subsysteminfo_componentinfo(
        conn, devinfo_id, subinfo_id,
    )?)
}

pub fn add_component_info_to_subsystem_info(
    conn: &MysqlConnection,
    devinfo_id: i32,
    subinfo_id: i32,
    cominfo_id: i32,
    quantity: i32,
) -> Result<usize> {
    get_subsystem_info(conn, subinfo_id)?;
    get_component_info(conn, cominfo_id)?;
    Ok(insert_subsysteminfo_componentinfo(
        conn,
        SubinfoCominfoInsert {
            device_info_id: devinfo_id,
            subsystem_info_id: subinfo_id,
            component_info_id: cominfo_id,
            quantity: quantity,
        },
    )?)
}

<<<<<<< HEAD


=======
pub fn remove_componentinfo_from_subsysteminfo(
    conn: &MysqlConnection,
    devinfo_id: i32,
    subinfo_id: i32,
    cominfo_id: i32,
) -> Result<usize> {
    Ok(delete_subsysteminfo_componentinfo(
        conn, devinfo_id, subinfo_id, cominfo_id,
    )?)
}

pub fn create_component_info(conn: &MysqlConnection, info: ComponentInfoInsert) -> Result<usize> {
    Ok(insert_component_info(conn, info)?)
}
>>>>>>> 953e92add90ce37ba9539463e99bd565ebf2a7b2
