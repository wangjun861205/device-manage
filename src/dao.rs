use super::model::*;
use std::error;

#[derive(Debug)]
pub struct Error(pub String);

use std::fmt::{self, Display, Formatter};

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", &self)
    }
}

impl error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

//======================================================DeviceInfo========================================================

pub trait DeviceInfoStorer {
    fn insert(&self, info: DeviceInfoInsert) -> Result<i32>;
    fn bulk_insert(&self, infos: &Vec<DeviceInfoInsert>) -> Result<usize>;
    fn delete(&self, id: i32) -> Result<usize>;
    fn bulk_delete(&self, query: DeviceInfoQuery) -> Result<usize>;
    fn update(&self, id: i32, upd: DeviceInfoUpdate) -> Result<usize>;
    fn get(&self, id: i32) -> Result<DeviceInfo>;
    fn query(&self, query: &DeviceInfoQuery) -> Result<(Vec<DeviceInfo>, i64)>;
    fn query_by_subsystem_info(&self, subinfoid: i32, query: DeviceInfoQuery) -> Result<(Vec<DeviceInfo>, i64)>;
    fn count(&self, query: DeviceInfoQuery) -> Result<i64>;
    fn is_exist(&self, id: i32) -> Result<bool>;
    fn detail(&self, id: i32) -> Result<(DeviceInfo, Vec<(SubsystemInfo, Vec<ComponentInfo>)>)>;
}

// ===================================================subsystem_info======================================================

pub trait SubsystemInfoStorer {
    fn insert(&self, info: SubsystemInfoInsert) -> Result<i32>;
    fn bulk_insert(&self, infos: &Vec<SubsystemInfoInsert>) -> Result<usize>;
    fn delete(&self, id: i32) -> Result<usize>;
    fn update(&self, id: i32, upd: SubsystemInfoUpdate) -> Result<usize>;
    fn get(&self, id: i32) -> Result<SubsystemInfo>;
    fn query(&self, query: &SubsystemInfoQuery) -> Result<(Vec<SubsystemInfo>, i64)>;
    fn query_by_device_info(&self, devinfoid: i32, query: SubsystemInfoQuery) -> Result<(Vec<SubsystemInfo>, i64)>;
    fn query_by_component_info(&self, comid: i32, query: SubsystemInfoQuery) -> Result<(Vec<SubsystemInfo>, i64)>;
    fn count(&self, query: SubsystemInfoQuery) -> Result<i64>;
    fn is_exist(&self, id: i32) -> Result<bool>;
}

// =======================================================component_info===================================================

pub trait ComponentInfoStorer {
    fn insert(&self, info: ComponentInfoInsert) -> Result<i32>;
    fn bulk_insert(&self, infos: &Vec<ComponentInfoInsert>) -> Result<usize>;
    fn delete(&self, id: i32) -> Result<usize>;
    fn update(&self, id: i32, upd: ComponentInfoUpdate) -> Result<usize>;
    fn get(&self, id: i32) -> Result<ComponentInfo>;
    fn query(&self, query: &ComponentInfoQuery) -> Result<(Vec<ComponentInfo>, i64)>;
    fn query_by_subsystem_info(&self, subinfoid: i32, query: ComponentInfoQuery) -> Result<(Vec<ComponentInfo>, i64)>;
    fn count(&self, query: ComponentInfoQuery) -> Result<i64>;
    fn is_exist(&self, id: i32) -> Result<bool>;
}

// =======================================================device====================================================

pub trait DeviceStorer {
    fn insert(&self, dev: DeviceInsert) -> Result<i32>;
    fn bulk_insert(&self, devs: &Vec<DeviceInsert>) -> Result<usize>;
    fn delete(&self, id: i32) -> Result<usize>;
    fn update(&self, id: i32, upd: DeviceUpdate) -> Result<usize>;
    fn get(&self, id: i32) -> Result<(Device, Vec<(Subsystem, Vec<Component>)>)>;
    fn query(&self, query: &DeviceQuery) -> Result<(Vec<(Device, Vec<(Subsystem, Vec<Component>)>)>, i64)>;
}

// ==================================================subsystem====================================================

pub trait SubsystemStorer {
    fn insert(&self, sub: SubsystemInsert) -> Result<i32>;
    fn bulk_insert(&self, subs: &Vec<SubsystemInsert>) -> Result<usize>;
    fn delete(&self, id: i32) -> Result<usize>;
    fn udpate(&self, id: i32, upd: SubsystemUpdate) -> Result<usize>;
    fn get(&self, id: i32) -> Result<(Device, Subsystem, Vec<Component>)>;
    fn query(&self, query: &SubsystemQuery) -> Result<(Vec<Subsystem>, i64)>;
}

// =================================================component=========================================================

pub trait ComponentStorer {
    fn insert(&self, com: ComponentInsert) -> Result<i32>;
    fn bulk_insert(&self, coms: &Vec<ComponentInsert>) -> Result<usize>;
    fn delete(&self, id: i32) -> Result<usize>;
    fn update(&self, id: i32, upd: ComponentUpdate) -> Result<usize>;
    fn get(&self, id: i32) -> Result<(Device, Subsystem, Component)>;
    fn query(&self, query: &ComponentQuery) -> Result<(Vec<Component>, i64)>;
}

// ==============================================================relations================================================

pub trait RelationStorer {
    fn insert_deviceinfo_subsysteminfo(&self, rel: DevinfoSubinfoInsert) -> Result<usize>;
    fn delete_deviceinfo_subsysteminfo(&self, devinfo_id: i32, subinfo_id: i32) -> Result<usize>;
    fn bulk_delete_deviceinfo_subsysteminfo(&self, devinfo_id: i32) -> Result<usize>;
    fn insert_subsysteminfo_componentinfo(&self, rel: SubinfoCominfoInsert) -> Result<usize>;
    fn delete_subsysteminfo_componentinfo(&self, devinfo_id: i32, subinfo_id: i32, cominfo_id: i32) -> Result<usize>;
    fn bulk_delete_subsysteminfo_componentinfo(&self, devinfo_id: i32, subinfo_id: i32) -> Result<usize>;
}
