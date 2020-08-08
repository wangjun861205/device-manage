use super::super::dao;
use super::super::dao::{ComponentInfoStorer, ComponentStorer, DeviceInfoStorer, DeviceStorer, RelationStorer, SubsystemInfoStorer, SubsystemStorer};
use super::super::model::*;
use super::super::schema::*;
use diesel;
use diesel::mysql::Mysql;
use diesel::sql_types::Integer;
use diesel::Connection;
use diesel::{select, BelongingToDsl, BoolExpressionMethods, ExpressionMethods, GroupedBy, MysqlConnection, QueryDsl, RunQueryDsl, TextExpressionMethods};
use r2d2;
use std::convert::From;
use std::fmt::{self, Display, Formatter};

no_arg_sql_function!(last_insert_id, Integer);

#[derive(Debug)]
pub struct Error(String);

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Error(format!("{}", e))
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Error(format!("{}", e))
    }
}

impl From<Error> for dao::Error {
    fn from(e: Error) -> Self {
        dao::Error(format!("{}", e))
    }
}

impl From<r2d2::Error> for dao::Error {
    fn from(e: r2d2::Error) -> Self {
        dao::Error(format!("{}", e))
    }
}

impl From<diesel::result::Error> for dao::Error {
    fn from(e: diesel::result::Error) -> Self {
        dao::Error(format!("{}", e))
    }
}

pub type Result<T> = std::result::Result<T, Error>;

//===========================================================device info===================================================

pub struct DeviceInfoRepository(MysqlConnection);

impl DeviceInfoRepository {
    pub fn new(conn: MysqlConnection) -> Self {
        DeviceInfoRepository(conn)
    }

    fn boxed_query(&self, query: &DeviceInfoQuery) -> device_info::BoxedQuery<Mysql> {
        let mut q = device_info::table.limit(query.size).offset((query.page - 1) * query.size).into_boxed();
        if let Some(v) = query.name.clone() {
            q = q.filter(device_info::name.like(format!("%{}%", v)));
        }
        if let Some(v) = query.model.clone() {
            q = q.filter(device_info::model.like(format!("%{}%", v)));
        }
        if let Some(v) = query.maintain_interval_begin {
            q = q.filter(device_info::maintain_interval.ge(v));
        }
        if let Some(v) = query.maintain_interval_end {
            q = q.filter(device_info::maintain_interval.lt(v));
        }
        q
    }
}

impl DeviceInfoStorer for DeviceInfoRepository {
    fn insert(&self, info: DeviceInfoInsert) -> dao::Result<i32> {
        diesel::insert_into(device_info::table).values(info).execute(&self.0)?;
        Ok(select(last_insert_id).first(&self.0)?)
    }

    fn bulk_insert(&self, infos: &Vec<DeviceInfoInsert>) -> dao::Result<usize> {
        Ok(diesel::insert_into(device_info::table).values(infos).execute(&self.0)?)
    }

    fn delete(&self, id: i32) -> dao::Result<usize> {
        Ok(diesel::delete(device_info::table.find(id)).execute(&self.0)?)
    }

    fn bulk_delete(&self, query: DeviceInfoQuery) -> dao::Result<usize> {
        let mut q = diesel::delete(device_info::table).into_boxed();
        if let Some(v) = query.name {
            q = q.filter(device_info::name.eq(v));
        }
        if let Some(v) = query.model {
            q = q.filter(device_info::model.eq(v));
        }
        if let Some(v) = query.maintain_interval_begin {
            q = q.filter(device_info::maintain_interval.ge(v));
        }
        if let Some(v) = query.maintain_interval_end {
            q = q.filter(device_info::maintain_interval.lt(v));
        }
        Ok(q.execute(&self.0)?)
    }

    fn update(&self, id: i32, upd: DeviceInfoUpdate) -> dao::Result<usize> {
        Ok(diesel::update(device_info::table).filter(device_info::id.eq(id)).set(upd).execute(&self.0)?)
    }

    fn get(&self, id: i32) -> dao::Result<DeviceInfo> {
        Ok(device_info::table.find(id).first(&self.0)?)
    }

    fn query(&self, query: &DeviceInfoQuery) -> dao::Result<(Vec<DeviceInfo>, i64)> {
        let q = self.boxed_query(query).limit(query.size).offset((query.page - 1) * query.size).load(&self.0)?;
        let cq = self.boxed_query(query).count().first(&self.0)?;
        Ok((q, cq))
    }

    fn query_by_subsystem_info(&self, subinfoid: i32, query: DeviceInfoQuery) -> dao::Result<(Vec<DeviceInfo>, i64)> {
        let t = device_info::table
            .inner_join(deviceinfo_subsysteminfo::table.inner_join(subsystem_info::table))
            .filter(subsystem_info::id.eq(subinfoid));
        let mut q = t.select(device_info::all_columns).limit(query.size).offset((query.page - 1) * query.size).into_boxed();
        let mut cq = t.count().into_boxed();
        if let Some(v) = query.name {
            q = q.filter(device_info::name.like(format!("%{}%", v)));
            cq = cq.filter(device_info::name.like(format!("%{}%", v)));
        }
        if let Some(v) = query.model {
            q = q.filter(device_info::model.like(format!("%{}%", v)));
            cq = cq.filter(device_info::model.like(format!("%{}%", v)));
        }
        if let Some(v) = query.maintain_interval_begin {
            q = q.filter(device_info::maintain_interval.ge(v));
            cq = cq.filter(device_info::maintain_interval.ge(v));
        }
        if let Some(v) = query.maintain_interval_end {
            q = q.filter(device_info::maintain_interval.lt(v));
            cq = cq.filter(device_info::maintain_interval.lt(v));
        }
        self.0.transaction(|| Ok((q.load(&self.0)?, cq.first(&self.0)?)))
    }

    fn count(&self, query: DeviceInfoQuery) -> dao::Result<i64> {
        let mut q = device_info::table.into_boxed();
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
        Ok(q.count().first(&self.0)?)
    }

    fn is_exist(&self, id: i32) -> dao::Result<bool> {
        Ok(device_info::table.filter(device_info::id.eq(id)).count().execute(&self.0)? > 0)
    }

    fn detail(&self, id: i32) -> dao::Result<(DeviceInfo, Vec<(SubsystemInfo, Vec<ComponentInfo>)>)> {
        let dev: DeviceInfo = device_info::table.find(id).first(&self.0)?;
        let subs: Result<Vec<(SubsystemInfo, Vec<ComponentInfo>)>> = DeviceinfoSubsysteminfo::belonging_to(&dev)
            .inner_join(subsystem_info::table)
            .select(subsystem_info::all_columns)
            .load(&self.0)?
            .into_iter()
            .map(|s: SubsystemInfo| {
                let coms = SubsysteminfoComponentinfo::belonging_to(&s)
                    .inner_join(component_info::table)
                    .filter(subsysteminfo_componentinfo::device_info_id.eq(dev.id))
                    .select(component_info::all_columns)
                    .load(&self.0)?;
                Ok((s, coms))
            })
            .collect();
        Ok((dev, subs?))
    }
}

pub struct SubsystemInfoRepository(MysqlConnection);

impl SubsystemInfoRepository {
    pub fn new(conn: MysqlConnection) -> Self {
        SubsystemInfoRepository(conn)
    }

    fn boxed_query(&self, query: &SubsystemInfoQuery) -> subsystem_info::BoxedQuery<Mysql> {
        let mut q = subsystem_info::table.limit(query.size).offset((query.size - 1) * query.size).into_boxed();
        if let Some(v) = query.name.clone() {
            q = q.filter(subsystem_info::name.like(format!("%{}%", v)));
        }
        if let Some(v) = query.maintain_interval_begin {
            q = q.filter(subsystem_info::maintain_interval.ge(v));
        }
        if let Some(v) = query.maintain_interval_end {
            q = q.filter(subsystem_info::maintain_interval.lt(v));
        }
        q
    }
}

impl SubsystemInfoStorer for SubsystemInfoRepository {
    fn insert(&self, info: SubsystemInfoInsert) -> dao::Result<i32> {
        diesel::insert_into(subsystem_info::table).values(info).execute(&self.0)?;
        Ok(select(last_insert_id).first(&self.0)?)
    }

    fn bulk_insert(&self, infos: &Vec<SubsystemInfoInsert>) -> dao::Result<usize> {
        Ok(diesel::insert_into(subsystem_info::table).values(infos).execute(&self.0)?)
    }

    fn delete(&self, id: i32) -> dao::Result<usize> {
        Ok(diesel::delete(subsystem_info::table.find(id)).execute(&self.0)?)
    }

    fn update(&self, id: i32, upd: SubsystemInfoUpdate) -> dao::Result<usize> {
        Ok(diesel::update(subsystem_info::table.find(id)).set(upd).execute(&self.0)?)
    }

    fn get(&self, id: i32) -> dao::Result<SubsystemInfo> {
        Ok(subsystem_info::table.find(id).first(&self.0)?)
    }

    fn query(&self, query: &SubsystemInfoQuery) -> dao::Result<(Vec<SubsystemInfo>, i64)> {
        let v = self.boxed_query(query).limit(query.size).offset((query.size - 1) * query.size).load(&self.0)?;
        let c = self.boxed_query(query).count().first(&self.0)?;
        Ok((v, c))
    }

    fn query_by_device_info(&self, devinfoid: i32, query: SubsystemInfoQuery) -> dao::Result<(Vec<SubsystemInfo>, i64)> {
        let t = device_info::table
            .inner_join(deviceinfo_subsysteminfo::table.inner_join(subsystem_info::table))
            .filter(device_info::id.eq(devinfoid));
        let mut q = t.select(subsystem_info::all_columns).limit(query.page).offset((query.page - 1) * query.size).into_boxed();
        let mut cq = t.count().into_boxed();
        if let Some(v) = query.name {
            q = q.filter(subsystem_info::name.like(format!("%{}%", v)));
            cq = cq.filter(subsystem_info::name.like(format!("%{}%", v)));
        }
        if let Some(v) = query.maintain_interval_begin {
            q = q.filter(subsystem_info::maintain_interval.ge(v));
            cq = cq.filter(subsystem_info::maintain_interval.ge(v));
        }
        if let Some(v) = query.maintain_interval_end {
            q = q.filter(subsystem_info::maintain_interval.lt(v));
            cq = cq.filter(subsystem_info::maintain_interval.lt(v));
        }
        self.0.transaction(|| Ok((q.load(&self.0)?, cq.first(&self.0)?)))
    }

    fn query_by_component_info(&self, comid: i32, query: SubsystemInfoQuery) -> dao::Result<(Vec<SubsystemInfo>, i64)> {
        let t = subsystem_info::table
            .inner_join(subsysteminfo_componentinfo::table.inner_join(component_info::table))
            .filter(component_info::id.eq(comid));
        let mut q = t.select(subsystem_info::all_columns).limit(query.size).offset((query.page - 1) * query.size).into_boxed();
        let mut cq = t.count().into_boxed();
        if let Some(v) = query.name {
            q = q.filter(subsystem_info::name.like(format!("%{}%", v)));
            cq = cq.filter(subsystem_info::name.like(format!("%{}%", v)));
        }
        if let Some(v) = query.maintain_interval_begin {
            q = q.filter(subsystem_info::maintain_interval.ge(v));
            cq = cq.filter(subsystem_info::maintain_interval.ge(v));
        }
        if let Some(v) = query.maintain_interval_end {
            q = q.filter(subsystem_info::maintain_interval.lt(v));
            cq = cq.filter(subsystem_info::maintain_interval.lt(v));
        }
        self.0.transaction(|| Ok((q.load(&self.0)?, cq.first(&self.0)?)))
    }

    fn count(&self, query: SubsystemInfoQuery) -> dao::Result<i64> {
        let mut q = subsystem_info::table.count().into_boxed();
        if let Some(v) = query.name {
            q = q.filter(subsystem_info::name.like(format!("%{}%", v)));
        }
        if let Some(v) = query.maintain_interval_begin {
            q = q.filter(subsystem_info::maintain_interval.ge(v));
        }
        if let Some(v) = query.maintain_interval_end {
            q = q.filter(subsystem_info::maintain_interval.lt(v));
        }
        Ok(q.first(&self.0)?)
    }

    fn is_exist(&self, id: i32) -> dao::Result<bool> {
        Ok(subsystem_info::table.filter(subsystem_info::id.eq(id)).count().execute(&self.0)? > 0)
    }
}

pub struct ComponentInfoRepository(MysqlConnection);

impl ComponentInfoRepository {
    pub fn new(conn: MysqlConnection) -> Self {
        ComponentInfoRepository(conn)
    }

    fn boxed_query(&self, query: &ComponentInfoQuery) -> component_info::BoxedQuery<Mysql> {
        let mut q = component_info::table.limit(query.size).offset((query.page - 1) * query.size).into_boxed();
        if let Some(v) = query.name.clone() {
            q = q.filter(component_info::name.like(format!("%{}%", v)));
        }
        if let Some(v) = query.model.clone() {
            q = q.filter(component_info::model.like(format!("%{}%", v)));
        }
        if let Some(v) = query.maintain_interval_begin {
            q = q.filter(component_info::maintain_interval.ge(v));
        }
        if let Some(v) = query.maintain_interval_end {
            q = q.filter(component_info::maintain_interval.lt(v));
        }
        q
    }
}

impl ComponentInfoStorer for ComponentInfoRepository {
    fn insert(&self, info: ComponentInfoInsert) -> dao::Result<i32> {
        diesel::insert_into(component_info::table).values(info).execute(&self.0)?;
        Ok(select(last_insert_id).first(&self.0)?)
    }

    fn bulk_insert(&self, infos: &Vec<ComponentInfoInsert>) -> dao::Result<usize> {
        Ok(diesel::insert_into(component_info::table).values(infos).execute(&self.0)?)
    }

    fn delete(&self, id: i32) -> dao::Result<usize> {
        Ok(diesel::delete(component_info::table.find(id)).execute(&self.0)?)
    }

    fn update(&self, id: i32, upd: ComponentInfoUpdate) -> dao::Result<usize> {
        Ok(diesel::update(component_info::table.find(id)).set(upd).execute(&self.0)?)
    }

    fn get(&self, id: i32) -> dao::Result<ComponentInfo> {
        Ok(device_info::table.find(id).first(&self.0)?)
    }

    fn query(&self, query: &ComponentInfoQuery) -> dao::Result<(Vec<ComponentInfo>, i64)> {
        let v = self.boxed_query(query).limit(query.size).offset((query.page - 1) * query.size).load(&self.0)?;
        let c = self.boxed_query(query).count().first(&self.0)?;
        Ok((v, c))
    }

    fn query_by_subsystem_info(&self, subinfoid: i32, query: ComponentInfoQuery) -> dao::Result<(Vec<ComponentInfo>, i64)> {
        let t = component_info::table
            .inner_join(subsysteminfo_componentinfo::table.inner_join(subsystem_info::table))
            .filter(subsystem_info::id.eq(subinfoid));
        let mut q = t.select(component_info::all_columns).limit(query.size).offset((query.page - 1) * query.size).into_boxed();
        let mut cq = t.count().into_boxed();
        if let Some(v) = query.name {
            q = q.filter(component_info::name.like(format!("%{}%", v)));
            cq = cq.filter(component_info::name.like(format!("%{}%", v)));
        }
        if let Some(v) = query.model {
            q = q.filter(component_info::model.like(format!("%{}%", v)));
            cq = cq.filter(component_info::model.like(format!("%{}%", v)));
        }
        if let Some(v) = query.maintain_interval_begin {
            q = q.filter(component_info::maintain_interval.ge(v));
            cq = cq.filter(component_info::maintain_interval.ge(v));
        }
        if let Some(v) = query.maintain_interval_end {
            q = q.filter(component_info::maintain_interval.lt(v));
            cq = cq.filter(component_info::maintain_interval.lt(v));
        }
        self.0.transaction(|| Ok((q.load(&self.0)?, cq.first(&self.0)?)))
    }

    fn count(&self, query: ComponentInfoQuery) -> dao::Result<i64> {
        let mut q = component_info::table.count().into_boxed();
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
        Ok(q.first(&self.0)?)
    }

    fn is_exist(&self, id: i32) -> dao::Result<bool> {
        Ok(component_info::table.filter(component_info::id.eq(id)).count().execute(&self.0)? > 0)
    }
}

pub struct DeviceRepository(MysqlConnection);

impl DeviceRepository {
    pub fn new(conn: MysqlConnection) -> Self {
        DeviceRepository(conn)
    }
    fn boxed_query(&self, query: &DeviceQuery) -> device::BoxedQuery<Mysql> {
        let mut q = device::table.into_boxed();
        if let Some(v) = query.name.clone() {
            q = q.filter(device::name.like(format!("%{}%", v)));
        }
        if let Some(v) = query.model.clone() {
            q = q.filter(device::model.like(format!("%{}%", v)));
        }
        if let Some(v) = query.maintain_interval_begin {
            q = q.filter(device::maintain_interval.ge(v));
        }
        if let Some(v) = query.maintain_interval_end {
            q = q.filter(device::maintain_interval.lt(v));
        }
        if let Some(v) = query.last_start_at_begin.as_ref() {
            q = q.filter(device::last_start_at.ge(v.0));
        }
        if let Some(v) = query.last_start_at_end.as_ref() {
            q = q.filter(device::last_start_at.lt(v.0));
        }
        if let Some(v) = query.last_stop_at_begin.as_ref() {
            q = q.filter(device::last_stop_at.ge(v.0));
        }
        if let Some(v) = query.last_stop_at_end.as_ref() {
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
        q
    }
}

impl DeviceStorer for DeviceRepository {
    fn insert(&self, dev: DeviceInsert) -> dao::Result<i32> {
        diesel::insert_into(device::table).values(dev).execute(&self.0)?;
        Ok(select(last_insert_id).first(&self.0)?)
    }

    fn bulk_insert(&self, devs: &Vec<DeviceInsert>) -> dao::Result<usize> {
        Ok(diesel::insert_into(device::table).values(devs).execute(&self.0)?)
    }

    fn delete(&self, id: i32) -> dao::Result<usize> {
        Ok(diesel::delete(device::table.find(id)).execute(&self.0)?)
    }

    fn update(&self, id: i32, upd: DeviceUpdate) -> dao::Result<usize> {
        Ok(diesel::update(device::table.find(id)).set(upd).execute(&self.0)?)
    }

    fn get(&self, id: i32) -> dao::Result<(Device, Vec<(Subsystem, Vec<Component>)>)> {
        let dev: Device = device::table.find(id).first(&self.0)?;
        let subs: Vec<Subsystem> = Subsystem::belonging_to(&dev).load(&self.0)?;
        let coms: Vec<Component> = Component::belonging_to(&subs).load(&self.0)?;
        let grouped_coms = coms.grouped_by(&subs);
        let grouped_subs_coms = subs.into_iter().zip(grouped_coms).collect();
        Ok((dev, grouped_subs_coms))
    }

    fn query(&self, query: &DeviceQuery) -> dao::Result<(Vec<(Device, Vec<(Subsystem, Vec<Component>)>)>, i64)> {
        let mut q = self.boxed_query(query);
        if let (Some(p), Some(s)) = (query.page, query.size) {
            q = q.limit(s).offset((p - 1) * s)
        }
        let c = self.boxed_query(query).count().first(&self.0)?;
        let devs: Vec<Device> = q.load(&self.0)?;
        let subs: Vec<Subsystem> = Subsystem::belonging_to(&devs).load(&self.0)?;
        let coms: Vec<Component> = Component::belonging_to(&subs).load(&self.0)?;
        let grouped_coms: Vec<Vec<Component>> = coms.grouped_by(&subs);
        let grouped_subs_coms: Vec<Vec<(Subsystem, Vec<Component>)>> = subs.into_iter().zip(grouped_coms).grouped_by(&devs);
        Ok((devs.into_iter().zip(grouped_subs_coms).collect(), c))
    }
}

pub struct SubsystemRepository(MysqlConnection);

impl SubsystemRepository {
    pub fn new(conn: MysqlConnection) -> Self {
        SubsystemRepository(conn)
    }

    fn boxed_query(&self, query: &SubsystemQuery) -> subsystem::BoxedQuery<Mysql> {
        let mut q = subsystem::table.into_boxed();
        if let Some(v) = query.subsystem_name.clone() {
            q = q.filter(subsystem::name.like(format!("%{}%", v)));
        }
        if let Some(v) = query.maintain_interval_begin {
            q = q.filter(subsystem::maintain_interval.ge(v));
        }
        if let Some(v) = query.maintain_interval_end {
            q = q.filter(subsystem::maintain_interval.lt(v));
        }
        q
    }
}

impl SubsystemStorer for SubsystemRepository {
    fn insert(&self, sub: SubsystemInsert) -> dao::Result<i32> {
        diesel::insert_into(subsystem::table).values(sub).execute(&self.0)?;
        Ok(select(last_insert_id).first(&self.0)?)
    }

    fn bulk_insert(&self, subs: &Vec<SubsystemInsert>) -> dao::Result<usize> {
        Ok(diesel::insert_into(subsystem::table).values(subs).execute(&self.0)?)
    }

    fn delete(&self, id: i32) -> dao::Result<usize> {
        Ok(diesel::delete(subsystem::table.find(id)).execute(&self.0)?)
    }

    fn udpate(&self, id: i32, upd: SubsystemUpdate) -> dao::Result<usize> {
        Ok(diesel::update(subsystem::table.find(id)).set(upd).execute(&self.0)?)
    }

    fn get(&self, id: i32) -> dao::Result<(Device, Subsystem, Vec<Component>)> {
        let dev_sub: (Device, Subsystem) = device::table.inner_join(subsystem::table).filter(subsystem::id.eq(id)).first(&self.0)?;
        let coms: Vec<Component> = Component::belonging_to(&dev_sub.1).load(&self.0)?;
        Ok((dev_sub.0, dev_sub.1, coms))
    }

    fn query(&self, query: &SubsystemQuery) -> dao::Result<(Vec<Subsystem>, i64)> {
        let mut q = self.boxed_query(query);
        if let (Some(p), Some(s)) = (query.page, query.size) {
            q = q.limit(s).offset((p - 1) * s)
        }
        let v = q.load(&self.0)?;
        let c = self.boxed_query(query).count().first(&self.0)?;
        Ok((v, c))
    }
}

pub struct ComponentRepository(MysqlConnection);

impl ComponentRepository {
    pub fn new(conn: MysqlConnection) -> Self {
        ComponentRepository(conn)
    }

    fn boxed_query(&self, query: &ComponentQuery) -> component::BoxedQuery<Mysql> {
        let mut q = component::table.into_boxed();
        if let Some(v) = query.name.clone() {
            q = q.filter(component::name.like(format!("%{}%", v)));
        }
        if let Some(v) = query.model.clone() {
            q = q.filter(component::model.like(format!("%{}%", v)));
        }
        if let Some(v) = query.maintain_interval_begin {
            q = q.filter(component::maintain_interval.ge(v));
        }
        if let Some(v) = query.maintain_interval_end {
            q = q.filter(component::maintain_interval.lt(v));
        }
        q
    }
}

impl ComponentStorer for ComponentRepository {
    fn insert(&self, com: ComponentInsert) -> dao::Result<i32> {
        diesel::insert_into(component::table).values(com).execute(&self.0)?;
        Ok(select(last_insert_id).first(&self.0)?)
    }

    fn bulk_insert(&self, coms: &Vec<ComponentInsert>) -> dao::Result<usize> {
        Ok(diesel::insert_into(component::table).values(coms).execute(&self.0)?)
    }

    fn delete(&self, id: i32) -> dao::Result<usize> {
        Ok(diesel::delete(component::table.find(id)).execute(&self.0)?)
    }

    fn update(&self, id: i32, upd: ComponentUpdate) -> dao::Result<usize> {
        Ok(diesel::update(component::table.find(id)).set(upd).execute(&self.0)?)
    }

    fn get(&self, id: i32) -> dao::Result<(Device, Subsystem, Component)> {
        let g: (Device, (Subsystem, Component)) = device::table
            .inner_join(subsystem::table.inner_join(component::table))
            .filter(component::id.eq(id))
            .first(&self.0)?;
        Ok((g.0, (g.1).0, (g.1).1))
    }

    fn query(&self, query: &ComponentQuery) -> dao::Result<(Vec<Component>, i64)> {
        let mut q = self.boxed_query(query);
        if let (Some(p), Some(s)) = (query.page, query.size) {
            q = q.limit(s).offset((p - 1) * s)
        }
        let v = q.load(&self.0)?;
        let c = self.boxed_query(query).count().first(&self.0)?;
        Ok((v, c))
    }
}

pub struct RelationRepository(MysqlConnection);

impl RelationRepository {
    pub fn new(conn: MysqlConnection) -> RelationRepository {
        RelationRepository(conn)
    }
}

impl RelationStorer for RelationRepository {
    fn insert_deviceinfo_subsysteminfo(&self, rel: DevinfoSubinfoInsert) -> dao::Result<usize> {
        Ok(diesel::insert_into(deviceinfo_subsysteminfo::table).values(rel).execute(&self.0)?)
    }

    fn delete_deviceinfo_subsysteminfo(&self, devinfo_id: i32, subinfo_id: i32) -> dao::Result<usize> {
        Ok(diesel::delete(deviceinfo_subsysteminfo::table)
            .filter(
                deviceinfo_subsysteminfo::device_info_id
                    .eq(devinfo_id)
                    .and(deviceinfo_subsysteminfo::subsystem_info_id.eq(subinfo_id)),
            )
            .execute(&self.0)?)
    }

    fn bulk_delete_deviceinfo_subsysteminfo(&self, devinfo_id: i32) -> dao::Result<usize> {
        Ok(diesel::delete(deviceinfo_subsysteminfo::table)
            .filter(deviceinfo_subsysteminfo::device_info_id.eq(devinfo_id))
            .execute(&self.0)?)
    }

    fn insert_subsysteminfo_componentinfo(&self, rel: SubinfoCominfoInsert) -> dao::Result<usize> {
        Ok(diesel::insert_into(subsysteminfo_componentinfo::table).values(rel).execute(&self.0)?)
    }

    fn delete_subsysteminfo_componentinfo(&self, devinfo_id: i32, subinfo_id: i32, cominfo_id: i32) -> dao::Result<usize> {
        Ok(diesel::delete(subsysteminfo_componentinfo::table)
            .filter(
                subsysteminfo_componentinfo::device_info_id
                    .eq(devinfo_id)
                    .and(subsysteminfo_componentinfo::subsystem_info_id.eq(subinfo_id))
                    .and(subsysteminfo_componentinfo::component_info_id.eq(cominfo_id)),
            )
            .execute(&self.0)?)
    }

    fn bulk_delete_subsysteminfo_componentinfo(&self, devinfo_id: i32, subinfo_id: i32) -> dao::Result<usize> {
        Ok(diesel::delete(subsysteminfo_componentinfo::table)
            .filter(
                subsysteminfo_componentinfo::device_info_id
                    .eq(devinfo_id)
                    .and(subsysteminfo_componentinfo::subsystem_info_id.eq(subinfo_id)),
            )
            .execute(&self.0)?)
    }
}
