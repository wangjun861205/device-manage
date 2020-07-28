use super::dao::*;
use super::model::*;
use diesel::mysql::MysqlConnection;
use diesel::result::Error;


pub fn add_subsystem_info_to_device_info(
    conn: &MysqlConnection,
    devinfo_id: i32,
    subinfo_id: i32,
) -> Result<usize, Error> {
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


pub fn add_component_info_to_subsystem_info(
    conn: &MysqlConnection,
    devinfo_id: i32,
    subinfo_id: i32,
    cominfo_id: i32,
    quantity: i32,
) -> Result<usize, Error> {
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

