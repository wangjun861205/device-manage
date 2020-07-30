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

pub fn insert_device(conn: &MysqlConnection, dev: DeviceInsert) -> QueryResult<usize> {
    Ok(diesel::insert_into(device::table)
        .values(dev)
        .execute(conn)?)
}

pub fn bulk_insert_device(conn: &MysqlConnection, devs: &Vec<DeviceInsert>) -> QueryResult<usize> {
    Ok(diesel::insert_into(device::table)
        .values(devs)
        .execute(conn)?)
}

pub fn delete_device(conn: &MysqlConnection, id: i32) -> QueryResult<usize> {
    Ok(diesel::delete(device::table.find(id)).execute(conn)?)
}

pub fn update_device(conn: &MysqlConnection, id: i32, upd: DeviceUpdate) -> QueryResult<usize> {
    Ok(diesel::update(device::table.find(id))
        .set(upd)
        .execute(conn)?)
}

pub fn get_device(
    conn: &MysqlConnection,
    id: i32,
) -> QueryResult<(Device, Vec<(Subsystem, Vec<Component>)>)> {
    let dev: Device = device::table.find(id).first(conn)?;
    let subs: Vec<Subsystem> = Subsystem::belonging_to(&dev).load(conn)?;
    let coms: Vec<Component> = Component::belonging_to(&subs).load(conn)?;
    let grouped_coms = coms.grouped_by(&subs);
    let grouped_subs_coms = subs.into_iter().zip(grouped_coms).collect();
    Ok((dev, grouped_subs_coms))
}

pub fn query_device(
    conn: &MysqlConnection,
    query: DeviceQuery,
) -> QueryResult<Vec<(Device, Vec<(Subsystem, Vec<Component>)>)>> {
    let mut q = device::table.into_boxed();
    if let Some(v) = query.name {
        q = q.filter(device::name.like(format!("%{}%", v)));
    }
    if let Some(v) = query.model {
        q = q.filter(device::model.like(format!("%{}%", v)));
    }
    if let Some(v) = query.maintain_interval_begin {
        q = q.filter(device::maintain_interval.ge(v));
    }
    if let Some(v) = query.maintain_interval_end {
        q = q.filter(device::maintain_interval.lt(v));
    }
    if let Some(v) = query.last_start_at_begin {
        q = q.filter(device::last_start_at.ge(v.0));
    }
    if let Some(v) = query.last_start_at_end {
        q = q.filter(device::last_start_at.lt(v.0));
    }
    if let Some(v) = query.last_stop_at_begin {
        q = q.filter(device::last_stop_at.ge(v.0));
    }
    if let Some(v) = query.last_stop_at_end {
        q = q.filter(device::last_stop_at.lt(v.0));
    }
    if let Some(v) = query.total_duration_begin {
        q = q.filter(device::total_duration.ge(v));
    }
    if let Some(v) = query.total_duration_end {
        q = q.filter(device::total_duration.lt(v));
    }
    if let Some(v) = query.status {
        q = q.filter(device::status.eq(v));
    }
    if let (Some(p), Some(s)) = (query.page, query.size) {
        q = q.limit(s).offset((p - 1) * s)
    }
    let devs: Vec<Device> = q.load(conn)?;
    let subs: Vec<Subsystem> = Subsystem::belonging_to(&devs).load(conn)?;
    let coms: Vec<Component> = Component::belonging_to(&subs).load(conn)?;
    let grouped_coms: Vec<Vec<Component>> = coms.grouped_by(&subs);
    let grouped_subs_coms: Vec<Vec<(Subsystem, Vec<Component>)>> =
        subs.into_iter().zip(grouped_coms).grouped_by(&devs);
    Ok(devs.into_iter().zip(grouped_subs_coms).collect())
}

// ==================================================subsystem====================================================

pub fn insert_subsystem(conn: &MysqlConnection, sub: SubsystemInsert) -> QueryResult<usize> {
    Ok(diesel::insert_into(subsystem::table)
        .values(sub)
        .execute(conn)?)
}

pub fn bulk_insert_subsystem(
    conn: &MysqlConnection,
    subs: &Vec<SubsystemInsert>,
) -> QueryResult<usize> {
    Ok(diesel::insert_into(subsystem::table)
        .values(subs)
        .execute(conn)?)
}

pub fn delete_subsystem(conn: &MysqlConnection, id: i32) -> QueryResult<usize> {
    Ok(diesel::delete(subsystem::table.find(id)).execute(conn)?)
}

pub fn udpate_subsystem(
    conn: &MysqlConnection,
    id: i32,
    upd: SubsystemUpdate,
) -> QueryResult<usize> {
    Ok(diesel::update(subsystem::table.find(id))
        .set(upd)
        .execute(conn)?)
}

pub fn get_subsystem(
    conn: &MysqlConnection,
    id: i32,
) -> QueryResult<(Device, Subsystem, Vec<Component>)> {
    let dev_sub: (Device, Subsystem) = device::table
        .inner_join(subsystem::table)
        .filter(subsystem::id.eq(id))
        .first(conn)?;
    let coms: Vec<Component> = Component::belonging_to(&dev_sub.1).load(conn)?;
    Ok((dev_sub.0, dev_sub.1, coms))
}

pub fn query_subsystem(
    conn: &MysqlConnection,
    query: SubsystemQuery,
) -> QueryResult<Vec<(Device, Subsystem, Vec<Component>)>> {
    let mut q = device::table.inner_join(subsystem::table).into_boxed();
    if let Some(v) = query.device_name {
        q = q.filter(device::name.like(format!("%{}%", v)));
    }
    if let Some(v) = query.device_model {
        q = q.filter(device::model.like(format!("%{}%", v)));
    }
    if let Some(v) = query.device_maintain_interval_begin {
        q = q.filter(device::maintain_interval.ge(v));
    }
    if let Some(v) = query.device_maintain_interval_end {
        q = q.filter(device::maintain_interval.lt(v));
    }
    if let Some(v) = query.device_last_start_at_begin {
        q = q.filter(device::last_start_at.ge(v.0));
    }
    if let Some(v) = query.device_last_start_at_end {
        q = q.filter(device::last_start_at.lt(v.0));
    }
    if let Some(v) = query.device_last_stop_at_begin {
        q = q.filter(device::last_stop_at.ge(v.0));
    }
    if let Some(v) = query.device_last_stop_at_end {
        q = q.filter(device::last_stop_at.lt(v.0));
    }
    if let Some(v) = query.device_total_duration_begin {
        q = q.filter(device::total_duration.ge(v));
    }
    if let Some(v) = query.device_total_duration_end {
        q = q.filter(device::total_duration.lt(v));
    }
    if let Some(v) = query.device_status {
        q = q.filter(device::status.eq(v));
    }
    if let Some(v) = query.maintain_interval_begin {
        q = q.filter(subsystem::maintain_interval.ge(v));
    }
    if let Some(v) = query.maintain_interval_end {
        q = q.filter(subsystem::maintain_interval.lt(v));
    }
    if let (Some(p), Some(s)) = (query.page, query.size) {
        q = q.limit(s).offset((p - 1) * s)
    }
    let dev_subs: Vec<(Device, Subsystem)> = q.load(conn)?;
    let subs: Vec<Subsystem> = dev_subs.iter().map(|d| d.1.clone()).collect();
    let coms: Vec<Vec<Component>> = Component::belonging_to(&subs).load(conn)?.grouped_by(&subs);
    Ok(dev_subs
        .into_iter()
        .zip(coms)
        .map(|t| ((t.0).0, (t.0).1, t.1))
        .collect())
}

// =================================================component=========================================================

pub fn insert_component(conn: &MysqlConnection, com: ComponentInsert) -> QueryResult<usize> {
    Ok(diesel::insert_into(component::table)
        .values(com)
        .execute(conn)?)
}

pub fn bulk_insert_component(
    conn: &MysqlConnection,
    coms: &Vec<ComponentInsert>,
) -> QueryResult<usize> {
    Ok(diesel::insert_into(component::table)
        .values(coms)
        .execute(conn)?)
}

pub fn delete_component(conn: &MysqlConnection, id: i32) -> QueryResult<usize> {
    Ok(diesel::delete(component::table.find(id)).execute(conn)?)
}

pub fn update_component(
    conn: &MysqlConnection,
    id: i32,
    upd: ComponentUpdate,
) -> QueryResult<usize> {
    Ok(diesel::update(component::table.find(id))
        .set(upd)
        .execute(conn)?)
}

pub fn get_component(
    conn: &MysqlConnection,
    id: i32,
) -> QueryResult<(Device, Subsystem, Component)> {
    let g: (Device, (Subsystem, Component)) = device::table
        .inner_join(subsystem::table.inner_join(component::table))
        .filter(component::id.eq(id))
        .first(conn)?;
    Ok((g.0, (g.1).0, (g.1).1))
}

pub fn query_component(
    conn: &MysqlConnection,
    query: ComponentQuery,
) -> QueryResult<Vec<(Device, Subsystem, Component)>> {
    let mut q = device::table
        .inner_join(subsystem::table.inner_join(component::table))
        .into_boxed();
    if let Some(v) = query.device_name {
        q = q.filter(device::name.like(format!("%{}%", v)))
    }
    if let Some(v) = query.device_model {
        q = q.filter(device::model.like(format!("%{}%", v)));
    }
    if let Some(v) = query.device_maintain_interval_begin {
        q = q.filter(device::maintain_interval.ge(v));
    }
    if let Some(v) = query.device_maintain_interval_end {
        q = q.filter(device::maintain_interval.lt(v));
    }
    if let Some(v) = query.subsystem_name {
        q = q.filter(subsystem::name.like(format!("%{}%", v)));
    }
    if let Some(v) = query.subsystem_maintain_interval_begin {
        q = q.filter(subsystem::maintain_interval.ge(v));
    }
    if let Some(v) = query.subsystem_maintain_interval_end {
        q = q.filter(subsystem::maintain_interval.lt(v));
    }
    if let Some(v) = query.name {
        q = q.filter(component::name.like(format!("%{}%", v)));
    }
    if let Some(v) = query.model {
        q = q.filter(component::model.like(format!("%{}%", v)));
    }
    if let Some(v) = query.maintain_interval_begin {
        q = q.filter(component::maintain_interval.ge(v));
    }
    if let Some(v) = query.maintain_interval_end {
        q = q.filter(component::maintain_interval.lt(v));
    }
    if let (Some(p), Some(s)) = (query.page, query.size) {
        q = q.limit(s).offset((p - 1) * s)
    }
    let g: Vec<(Device, (Subsystem, Component))> = q.load(conn)?;
    Ok(g.into_iter().map(|t| (t.0, (t.1).0, (t.1).1)).collect())
}

// ==============================================================relations================================================

pub fn insert_deviceinfo_subsysteminfo(
    conn: &MysqlConnection,
    rel: DevinfoSubinfoInsert,
) -> QueryResult<usize> {
    Ok(diesel::insert_into(deviceinfo_subsysteminfo::table)
        .values(rel)
        .execute(conn)?)
}

pub fn delete_deviceinfo_subsysteminfo(
    conn: &MysqlConnection,
    devinfo_id: i32,
    subinfo_id: i32,
) -> QueryResult<usize> {
    Ok(diesel::delete(deviceinfo_subsysteminfo::table)
        .filter(
            deviceinfo_subsysteminfo::device_info_id
                .eq(devinfo_id)
                .and(deviceinfo_subsysteminfo::subsystem_info_id.eq(subinfo_id)),
        )
        .execute(conn)?)
}

pub fn bulk_delete_deviceinfo_subsysteminfo(
    conn: &MysqlConnection,
    devinfo_id: i32,
) -> QueryResult<usize> {
    Ok(diesel::delete(deviceinfo_subsysteminfo::table)
        .filter(deviceinfo_subsysteminfo::device_info_id.eq(devinfo_id))
        .execute(conn)?)
}

pub fn insert_subsysteminfo_componentinfo(
    conn: &MysqlConnection,
    rel: SubinfoCominfoInsert,
) -> QueryResult<usize> {
    Ok(diesel::insert_into(subsysteminfo_componentinfo::table)
        .values(rel)
        .execute(conn)?)
}

pub fn delete_subsysteminfo_componentinfo(
    conn: &MysqlConnection,
    devinfo_id: i32,
    subinfo_id: i32,
    cominfo_id: i32,
) -> QueryResult<usize> {
    Ok(diesel::delete(subsysteminfo_componentinfo::table)
        .filter(
            subsysteminfo_componentinfo::device_info_id
                .eq(devinfo_id)
                .and(subsysteminfo_componentinfo::subsystem_info_id.eq(subinfo_id))
                .and(subsysteminfo_componentinfo::component_info_id.eq(cominfo_id)),
        )
        .execute(conn)?)
}

pub fn bulk_delete_subsysteminfo_componentinfo(
    conn: &MysqlConnection,
    devinfo_id: i32,
    subinfo_id: i32,
) -> QueryResult<usize> {
    Ok(diesel::delete(subsysteminfo_componentinfo::table)
        .filter(
            subsysteminfo_componentinfo::device_info_id
                .eq(devinfo_id)
                .and(subsysteminfo_componentinfo::subsystem_info_id.eq(subinfo_id)),
        )
        .execute(conn)?)
}
