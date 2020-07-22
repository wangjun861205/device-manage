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
use super::model::*;
use super::MysqlConn;
use super::schema::*;
use diesel::{ RunQueryDsl, ExpressionMethods, QueryDsl, TextExpressionMethods, BelongingToDsl, GroupedBy };
use diesel::mysql::MysqlConnection;


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

// ==============================================device_info=========================================================

pub fn insert_device_info(conn: MysqlConn, info: DeviceInfoInsert) -> diesel::result::QueryResult<usize> {
    Ok(diesel::insert_into(device_info::table).values(info).execute(&*conn)?)
}

pub fn delete_device_info(conn: MysqlConn, id: i32) -> diesel::result::QueryResult<usize> {
    Ok(diesel::delete(device_info::table.find(id)).execute(&*conn)?)
}

pub fn update_device_info(conn: MysqlConn, id: i32, upd: DeviceInfoUpdate) -> diesel::result::QueryResult<usize> {
    Ok(diesel::update(device_info::table).filter(device_info::id.eq(id)).set(upd).execute(&*conn)?)
}

pub fn get_device_info(conn: MysqlConn, id: i32) -> diesel::result::QueryResult<DeviceInfo> {
    Ok(device_info::table.find(id).first(&*conn)?)
}

pub fn query_device_infos(conn: MysqlConn, query: DeviceInfoQuery) -> diesel::result::QueryResult<Vec<DeviceInfo>> {
    let q = device_info::table.into_boxed();
    if let Some(v) = query.name {
        q = q.filter(device_info::name.like(format!("%{}%", v)));
    }
    if let Some(v) = query.model {
        q = q.filter(device_info::model.like(format!("%{}%", v)));
    }
    if let Some(v) = query.maintain_interval_begin {
        q = q.filter(device_info::maintain_interval.ge(v));
    }
    if let Some(v) = query.maintain_interval_end {
        q = q.filter(device_info::maintain_interval.lt(v));
    }
    if let (Some(p), Some(s))  = (query.page, query.size) {
        q = q.limit(s).offset((p-1)*s)
    }
    Ok(q.load(&*conn)?)
}

// ===================================================subsystem_info======================================================

pub fn insert_subsystem_info(conn: &diesel::mysql::MysqlConnection, info: SubsystemInfoInsert) -> diesel::result::QueryResult<usize> {
    Ok(diesel::insert_into(subsystem_info::table).values(info).execute(conn)?)
}

pub fn delete_subsystem_info(conn: &diesel::mysql::MysqlConnection, id: i32) -> diesel::result::QueryResult<usize> {
    Ok(diesel::delete(subsystem_info::table.find(id)).execute(conn)?)
}

pub fn update_subsystem_info(conn: &diesel::mysql::MysqlConnection, id: i32, upd: SubsystemInfoUpdate) -> diesel::result::QueryResult<usize> {
    Ok(diesel::update(subsystem_info::table.find(id)).set(upd).execute(conn)?)
}

pub fn get_subsystem_info(conn: &diesel::mysql::MysqlConnection, id: i32) -> diesel::result::QueryResult<(DeviceInfo, SubsystemInfo)> {
    Ok(device_info::table.inner_join(deviceinfo_subsysteminfo::table.inner_join(subsystem_info::table)).filter(subsystem_info::id.eq(id))
    .select((device_info::all_columns, subsystem_info::all_columns))
    .first(conn)?)
}

pub fn query_subsystem_info(conn: &diesel::mysql::MysqlConnection, query: SubsystemInfoQuery) -> diesel::result::QueryResult<Vec<(SubsystemInfo, DeviceInfo)>> {
    let mut q = device_info::table.inner_join(deviceinfo_subsysteminfo::table.inner_join(subsystem_info::table))
    .select((subsystem_info::all_columns, device_info::all_columns))
    .into_boxed();
    if let Some(v) = query.device_info_name {
        q = q.filter(device_info::name.like(format!("%{}%", v)));
    }
    if let Some(v) = query.device_info_model {
        q = q.filter(device_info::model.like(format!("%{}%", v)));
    }
    if let Some(v) = query.device_info_maintain_interval_begin {
        q = q.filter(device_info::maintain_interval.ge(v));
    }
    if let Some(v) = query.device_info_maintain_interval_end {
        q = q.filter(device_info::maintain_interval.lt(v));
    }
    if let Some(v) = query.name {
        q = q.filter(subsystem_info::name.like(format!("%{}%", v)));
    }
    if let Some(v) = query.maintain_interval_begin {
        q = q.filter(subsystem_info::maintain_interval.ge(v));
    }
    if let Some(v) = query.maintain_interval_end {
        q = q.filter(subsystem_info::maintain_interval.lt(v));
    }
    if let (Some(p), Some(s)) = (query.page, query.size) {
        q = q.limit(s).offset((p-1)*s)
    }
    Ok(q.load(conn)?)
}

// =======================================================component_info===================================================

pub fn insert_component_info(conn: &MysqlConnection, info: ComponentInfoInsert) -> diesel::result::QueryResult<usize> {
    Ok(diesel::insert_into(component_info::table).values(info).execute(conn)?)
}

pub fn delete_component_info(conn: &MysqlConnection, id: i32) -> diesel::result::QueryResult<usize> {
    Ok(diesel::delete(component_info::table.find(id)).execute(conn)?)
}

pub fn update_component_info(conn: &MysqlConnection, id: i32, upd: ComponentInfoUpdate) -> diesel::result::QueryResult<usize> {
    Ok(diesel::update(component_info::table.find(id)).set(upd).execute(conn)?)
}

pub fn get_component_info(conn: &MysqlConnection, id: i32) -> diesel::result::QueryResult<(DeviceInfo, SubsystemInfo, ComponentInfo)> {
    Ok(
        device_info::table.inner_join(
            deviceinfo_subsysteminfo::table.inner_join(
                subsystem_info::table.inner_join(
                    subsysteminfo_componentinfo::table.inner_join(
                        component_info::table))))
                        .filter(component_info::id.eq(id))
                        .select((device_info::all_columns, subsystem_info::all_columns, component_info::all_columns)).first(conn)?)
}

pub fn query_component_info(conn: &MysqlConnection, query: ComponentInfoQuery) -> diesel::result::QueryResult<Vec<(DeviceInfo, SubsystemInfo, ComponentInfo)>> {
        let mut q = device_info::table.inner_join(
            deviceinfo_subsysteminfo::table.inner_join(
                subsystem_info::table.inner_join(
                    subsysteminfo_componentinfo::table.inner_join(
                        component_info::table
                    )
                )
            )
        )
        .select((device_info::all_columns, subsystem_info::all_columns, component_info::all_columns))
        .into_boxed();
        if let Some(v) = query.device_info_name {
            q = q.filter(device_info::name.like(format!("%{}%", v)));
        }
        if let Some(v) = query.device_info_model {
            q = q.filter(device_info::model.like(format!("%{}%", v)));   
        }
        if let Some(v) = query.device_info_maintain_interval_begin {
            q = q.filter(device_info::maintain_interval.ge(v));
        }
        if let Some(v) = query.device_info_maintain_interval_end {
            q = q.filter(device_info::maintain_interval.lt(v));
        }
        if let Some(v) = query.subsystem_info_name {
            q = q.filter(subsystem_info::name.like(format!("%{}%", v)));
        }
        if let Some(v) = query.subsystem_info_maintain_interval_begin {
            q = q.filter(subsystem_info::maintain_interval.ge(v));
        }
        if let Some(v) = query.subsystem_info_maintain_interval_end {
            q = q.filter(subsystem_info::maintain_interval.lt(v));
        }
        if let Some(v) = query.name {
            q = q.filter(component_info::name.like(format!("%{}%", v)));
        }
        if let Some(v) = query.model {
            q = q.filter(component_info::model.like(format!("%{}%", v)));
        }
        if let Some(v) = query.maintain_interval_begin {
            q = q.filter(component_info::maintain_interval.ge(v));
        }
        if let Some(v) = query.maintain_interval_end {
            q = q.filter(component_info::maintain_interval.lt(v));
        }
        if let (Some(p), Some(s)) = (query.page, query.size) {
            q = q.limit(s).offset((p-1)*s)
        }
        Ok(q.load(conn)?)
}

// =======================================================device====================================================

pub fn insert_device(conn: &MysqlConnection, dev: DeviceInsert) -> diesel::result::QueryResult<usize> {
    Ok(diesel::insert_into(device::table).values(dev).execute(conn)?)
}

pub fn delete_device(conn: &MysqlConnection, id: i32) -> diesel::result::QueryResult<usize> {
    Ok(diesel::delete(device::table.find(id)).execute(conn)?)
}

pub fn update_device(conn: &MysqlConnection, id: i32, upd: DeviceUpdate) -> diesel::result::QueryResult<usize> {
    Ok(diesel::update(device::table.find(id)).set(upd).execute(conn)?)
}

pub fn get_device(conn: &MysqlConnection, id: i32) -> diesel::result::QueryResult<(Device, Vec<(Subsystem, Vec<Component>)>)> {
    let dev: Device = device::table.find(id).first(conn)?;
    let subs: Vec<Subsystem> = Subsystem::belonging_to(&dev).load(conn)?;
    let coms: Vec<Component> = Component::belonging_to(&subs).load(conn)?;
    let grouped_coms = coms.grouped_by(&subs);
    let grouped_subs_coms = subs.into_iter().zip(grouped_coms).collect();
    Ok((dev, grouped_subs_coms))
}

pub fn query_device(conn: &MysqlConnection, query: DeviceQuery) -> diesel::result::QueryResult<Vec<(Device, Vec<(Subsystem, Vec<Component>)>)>> {
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
        q = q.limit(s).offset((p-1)*s)
    }
    let devs: Vec<Device> = q.load(conn)?;
    let subs: Vec<Subsystem> = Subsystem::belonging_to(&devs).load(conn)?;
    let coms: Vec<Component> = Component::belonging_to(&subs).load(conn)?;
    let grouped_coms: Vec<Vec<Component>> = coms.grouped_by(&subs);
    let grouped_subs_coms: Vec<Vec<(Subsystem, Vec<Component>)>> = subs
    .into_iter()
    .zip(grouped_coms)
    .grouped_by(&devs);
    Ok(devs.into_iter().zip(grouped_subs_coms).collect())
}



// ==================================================subsystem====================================================



pub fn insert_subsystem(conn: &MysqlConnection, sub: SubsystemInsert) -> diesel::result::QueryResult<usize> {
    Ok(diesel::insert_into(subsystem::table).values(sub).execute(conn)?)
}

pub fn delete_subsystem(conn: &MysqlConnection, id: i32) -> diesel::result::QueryResult<usize> {
    Ok(diesel::delete(subsystem::table.find(id)).execute(conn)?)
}

pub fn udpate_subsystem(conn: &MysqlConnection, id: i32, upd: SubsystemUpdate) -> diesel::result::QueryResult<usize> {
    Ok(diesel::update(subsystem::table.find(id)).set(upd).execute(conn)?)
}

pub fn get_subsystem(conn: &MysqlConnection, id: i32) -> diesel::result::QueryResult<(Device, Subsystem, Vec<Component>)> {
    let dev_sub: (Device, Subsystem) = device::table.inner_join(subsystem::table).filter(subsystem::id.eq(id)).first(conn)?;
    let coms: Vec<Component> = Component::belonging_to(&dev_sub.1).load(conn)?;
    Ok((dev_sub.0, dev_sub.1, coms))
}

pub fn query_subsystem(conn: &MysqlConnection, query: SubsystemQuery) -> diesel::result::QueryResult<Vec<(Device, Subsystem, Vec<Component>)>> {
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
        q = q.limit(s).offset((p-1)*s)
    }
    let dev_subs: Vec<(Device, Subsystem)> = q.load(conn)?;
    let devs: Vec<Device> = dev_subs.iter().map(|d| d.0.clone()).collect();
    let subs: Vec<Subsystem> = dev_subs.iter().map(|d| d.1.clone()).collect();
    let coms: Vec<Vec<Component>> = Component::belonging_to(&subs).load(conn)?.grouped_by(&subs);
    Ok(dev_subs.into_iter().zip(coms).map(|t| ((t.0).0, (t.0).1, t.1)).collect())
}

// =================================================component=========================================================

pub fn insert_component(conn: &MysqlConnection, com: ComponentInsert) -> diesel::result::QueryResult<usize> {
    Ok(diesel::insert_into(component::table).values(com).execute(conn)?)
}

pub fn delete_component(conn: &MysqlConnection, id: i32) -> diesel::result::QueryResult<usize> {
    Ok(diesel::delete(component::table.find(id)).execute(conn)?)
}

pub fn update_component(conn: &MysqlConnection, id: i32, upd: ComponentUpdate) -> diesel::result::QueryResult<usize> {
    Ok(diesel::update(component::table.find(id)).set(upd).execute(conn)?)
}

pub fn get_component(conn: &MysqlConnection, id: i32) -> diesel::result::QueryResult<(Device, Subsystem, Component)> {
    let g: (Device, (Subsystem, Component)) = device::table.inner_join(subsystem::table.inner_join(component::table)).filter(component::id.eq(id)).first(conn)?;
    Ok((g.0, (g.1).0, (g.1).1))
}

pub fn query_component(conn: &MysqlConnection, query: ComponentQuery) -> diesel::result::QueryResult<Vec<(Device, Subsystem, Component)>> {
    let mut q = device::table.inner_join(subsystem::table.inner_join(component::table)).into_boxed();
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
        q = q.limit(s).offset((p-1)*s)
    }
    let g: Vec<(Device, (Subsystem, Component))> = q.load(conn)?;
    Ok(g.into_iter().map(|t| (t.0, (t.1).0, (t.1).1)).collect())
}