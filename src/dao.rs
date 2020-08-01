use super::model::*;
use super::schema::*;
use diesel::mysql::MysqlConnection;
use diesel::result::QueryResult;
use diesel::Connection;
use diesel::{
    BelongingToDsl, BoolExpressionMethods, ExpressionMethods, GroupedBy, QueryDsl, RunQueryDsl,
    TextExpressionMethods,
};

#[derive(Debug)]
pub struct Error(pub String);

use std::fmt::{self, Display, Formatter};

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", &self)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

//======================================================DeviceInfo========================================================

pub trait DeviceInfoStorer {
    fn insert(&self, info: DeviceInfoInsert) -> Result<usize>;
    fn bulk_insert(&self, infos: &Vec<DeviceInfoInsert>) -> Result<usize>;
    fn delete(&self, id: i32) -> Result<usize>;
    fn bulk_delete(&self, query: DeviceInfoQuery) -> Result<usize>;
    fn update(&self, id: i32, upd: DeviceInfoUpdate) -> Result<usize>;
    fn get(&self, id: i32) -> Result<DeviceInfo>;
    fn query(&self, query: DeviceInfoQuery) -> Result<(Vec<DeviceInfo>, i64)>;
    fn query_by_subsystem_info(
        &self,
        subinfoid: i32,
        query: DeviceInfoQuery,
    ) -> Result<(Vec<DeviceInfo>, i64)>;
    fn count(&self, query: DeviceInfoQuery) -> Result<i64>;
    fn is_exist(&self, id: i32) -> Result<bool>;
}

// ===================================================subsystem_info======================================================

pub trait SubsystemInfoStorer {
    fn insert(&self, info: SubsystemInfoInsert) -> Result<usize>;

    fn bulk_insert(&self, infos: &Vec<SubsystemInfoInsert>) -> Result<usize>;

    fn delete(&self, id: i32) -> Result<usize>;
    fn update(&self, id: i32, upd: SubsystemInfoUpdate) -> Result<usize>;
    fn get(&self, id: i32) -> Result<SubsystemInfo>;
    fn query(&self, query: SubsystemInfoQuery) -> Result<(Vec<SubsystemInfo>, i64)>;
    fn query_by_device_info(
        &self,
        devinfoid: i32,
        query: SubsystemInfoQuery,
    ) -> Result<(Vec<SubsystemInfo>, i64)>;
    fn query_by_component_info(
        &self,
        comid: i32,
        query: SubsystemInfoQuery,
    ) -> Result<(Vec<SubsystemInfo>, i64)>;
    fn count(&self, query: SubsystemInfoQuery) -> Result<i64>;
    fn is_exist(&self, id: i32) -> Result<bool>;
}

// =======================================================component_info===================================================

pub trait ComponentInfoStorer {
    fn insert(&self, info: ComponentInfoInsert) -> Result<usize>;

    fn bulk_insert(&self, infos: &Vec<ComponentInfoInsert>) -> Result<usize>;

    fn delete(&self, id: i32) -> Result<usize>;

    fn update(&self, id: i32, upd: ComponentInfoUpdate) -> Result<usize>;

    fn get(&self, id: i32) -> Result<ComponentInfo>;

    fn query(&self, query: ComponentInfoQuery) -> Result<(Vec<ComponentInfo>, i64)>;

    fn query_by_subsystem_info(
        &self,
        subinfoid: i32,
        query: ComponentInfoQuery,
    ) -> Result<(Vec<ComponentInfo>, i64)>;

    fn count(&self, query: ComponentInfoQuery) -> Result<i64>;

    fn is_exist(&self, id: i32) -> Result<bool>;
}

// =======================================================device====================================================

pub trait DeviceStorer {
    fn insert_device(&self, dev: DeviceInsert) -> Result<usize>;

    fn bulk_insert_device(&self, devs: &Vec<DeviceInsert>) -> Result<usize>;

    fn delete_device(&self, id: i32) -> Result<usize>;

    fn update_device(&self, id: i32, upd: DeviceUpdate) -> Result<usize>;

    fn get_device(&self, id: i32) -> Result<(Device, Vec<(Subsystem, Vec<Component>)>)>;

    fn query_device(
        &self,
        query: DeviceQuery,
    ) -> Result<Vec<(Device, Vec<(Subsystem, Vec<Component>)>)>>;
}

// ==================================================subsystem====================================================

pub trait SubsystemStorer {
    fn insert_subsystem(&self, sub: SubsystemInsert) -> Result<usize>;

    fn bulk_insert_subsystem(&self, subs: &Vec<SubsystemInsert>) -> Result<usize>;

    fn delete_subsystem(&self, id: i32) -> Result<usize>;

    fn udpate_subsystem(&self, id: i32, upd: SubsystemUpdate) -> Result<usize>;

    fn get_subsystem(&self, id: i32) -> Result<(Device, Subsystem, Vec<Component>)>;

    fn query_subsystem(
        &self,
        query: SubsystemQuery,
    ) -> Result<Vec<(Device, Subsystem, Vec<Component>)>>;
}

// =================================================component=========================================================

pub trait ComponentStorer {
    fn insert_component(&self, com: ComponentInsert) -> Result<usize>;

    fn bulk_insert_component(&self, coms: &Vec<ComponentInsert>) -> Result<usize>;

    fn delete_component(&self, id: i32) -> Result<usize>;

    fn update_component(&self, id: i32, upd: ComponentUpdate) -> Result<usize>;

    fn get_component(&self, id: i32) -> Result<(Device, Subsystem, Component)>;

    fn query_component(&self, query: ComponentQuery)
        -> Result<Vec<(Device, Subsystem, Component)>>;
}

// ==============================================================relations================================================

pub trait RelationStorer {
    fn insert_deviceinfo_subsysteminfo(&self, rel: DevinfoSubinfoInsert) -> Result<usize>;

    fn delete_deviceinfo_subsysteminfo(&self, devinfo_id: i32, subinfo_id: i32) -> Result<usize>;

    fn bulk_delete_deviceinfo_subsysteminfo(&self, devinfo_id: i32) -> Result<usize>;

    fn insert_subsysteminfo_componentinfo(&self, rel: SubinfoCominfoInsert) -> Result<usize>;

    fn delete_subsysteminfo_componentinfo(
        &self,
        devinfo_id: i32,
        subinfo_id: i32,
        cominfo_id: i32,
    ) -> Result<usize>;

    fn bulk_delete_subsysteminfo_componentinfo(
        &self,
        devinfo_id: i32,
        subinfo_id: i32,
    ) -> Result<usize>;
}
