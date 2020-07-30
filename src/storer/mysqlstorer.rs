use super::super::dao;
use super::super::dao::{ComponentInfoStorer, DeviceInfoStorer, SubsystemInfoStorer};
use super::super::model::*;
use super::super::schema::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::Connection;
use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl, TextExpressionMethods};
use r2d2;
use std::convert::From;
use std::fmt::{self, Display, Formatter};
use std::ops::Deref;

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

pub struct DeviceInfoRepository {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl DeviceInfoRepository {
    pub fn new(pool: Pool<ConnectionManager<MysqlConnection>>) -> Self {
        DeviceInfoRepository { pool }
    }
}

impl DeviceInfoStorer for DeviceInfoRepository {
    fn insert(&self, info: DeviceInfoInsert) -> dao::Result<usize> {
        Ok(diesel::insert_into(device_info::table)
            .values(info)
            .execute(&self.pool.get()?)?)
    }

    fn bulk_insert(&self, infos: &Vec<DeviceInfoInsert>) -> dao::Result<usize> {
        Ok(diesel::insert_into(device_info::table)
            .values(infos)
            .execute(&self.pool.get()?)?)
    }

    fn delete(&self, id: i32) -> dao::Result<usize> {
        Ok(diesel::delete(device_info::table.find(id)).execute(&self.pool.get()?)?)
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
        Ok(q.execute(&self.pool.get()?)?)
    }

    fn update(&self, id: i32, upd: DeviceInfoUpdate) -> dao::Result<usize> {
        Ok(diesel::update(device_info::table)
            .filter(device_info::id.eq(id))
            .set(upd)
            .execute(&self.pool.get()?)?)
    }

    fn get(&self, id: i32) -> dao::Result<DeviceInfo> {
        Ok(device_info::table.find(id).first(&self.pool.get()?)?)
    }

    fn query(&self, query: DeviceInfoQuery) -> dao::Result<(Vec<DeviceInfo>, i64)> {
        let mut q = device_info::table
            .limit(query.size)
            .offset((query.page - 1) * query.size)
            .into_boxed();
        let mut cq = device_info::table.count().into_boxed();
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
        self.pool
            .get()?
            .transaction(|| Ok((q.load(&self.pool.get()?)?, cq.first(&self.pool.get()?)?)))
    }

    fn query_by_subsystem_info(
        &self,
        subinfoid: i32,
        query: DeviceInfoQuery,
    ) -> dao::Result<(Vec<DeviceInfo>, i64)> {
        let t = device_info::table
            .inner_join(deviceinfo_subsysteminfo::table.inner_join(subsystem_info::table))
            .filter(subsystem_info::id.eq(subinfoid));
        let mut q = t
            .select(device_info::all_columns)
            .limit(query.size)
            .offset((query.page - 1) * query.size)
            .into_boxed();
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
        self.pool
            .get()?
            .transaction(|| Ok((q.load(&self.pool.get()?)?, cq.first(&self.pool.get()?)?)))
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
        Ok(q.count().first(&self.pool.get()?)?)
    }

    fn is_exist(&self, id: i32) -> dao::Result<bool> {
        Ok(device_info::table
            .filter(device_info::id.eq(id))
            .count()
            .execute(&self.pool.get()?)?
            > 0)
    }
}

pub struct SubsystemInfoRepository {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl SubsystemInfoRepository {
    pub fn new(pool: Pool<ConnectionManager<MysqlConnection>>) -> Self {
        SubsystemInfoRepository { pool }
    }
}

impl SubsystemInfoStorer for SubsystemInfoRepository {
    fn insert(&self, info: SubsystemInfoInsert) -> dao::Result<usize> {
        Ok(diesel::insert_into(subsystem_info::table)
            .values(info)
            .execute(&self.pool.get()?)?)
    }

    fn bulk_insert(&self, infos: &Vec<SubsystemInfoInsert>) -> dao::Result<usize> {
        Ok(diesel::insert_into(subsystem_info::table)
            .values(infos)
            .execute(&self.pool.get()?)?)
    }

    fn delete(&self, id: i32) -> dao::Result<usize> {
        Ok(diesel::delete(subsystem_info::table.find(id)).execute(&self.pool.get()?)?)
    }

    fn update(&self, id: i32, upd: SubsystemInfoUpdate) -> dao::Result<usize> {
        Ok(diesel::update(subsystem_info::table.find(id))
            .set(upd)
            .execute(&self.pool.get()?)?)
    }

    fn get(&self, id: i32) -> dao::Result<SubsystemInfo> {
        Ok(subsystem_info::table.find(id).first(&self.pool.get()?)?)
    }

    fn query(&self, query: SubsystemInfoQuery) -> dao::Result<(Vec<SubsystemInfo>, i64)> {
        let mut q = subsystem_info::table
            .limit(query.size)
            .offset((query.size - 1) * query.size)
            .into_boxed();
        let mut cq = subsystem_info::table.count().into_boxed();
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
        self.pool
            .get()?
            .transaction(|| Ok((q.load(&self.pool.get()?)?, cq.first(&self.pool.get()?)?)))
    }

    fn query_by_device_info(
        &self,
        devinfoid: i32,
        query: SubsystemInfoQuery,
    ) -> dao::Result<(Vec<SubsystemInfo>, i64)> {
        let t = device_info::table
            .inner_join(deviceinfo_subsysteminfo::table.inner_join(subsystem_info::table))
            .filter(device_info::id.eq(devinfoid));
        let mut q = t
            .select(subsystem_info::all_columns)
            .limit(query.page)
            .offset((query.page - 1) * query.size)
            .into_boxed();
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
        self.pool
            .get()?
            .transaction(|| Ok((q.load(&self.pool.get()?)?, cq.first(&self.pool.get()?)?)))
    }

    fn query_by_component_info(
        &self,
        comid: i32,
        query: SubsystemInfoQuery,
    ) -> dao::Result<(Vec<SubsystemInfo>, i64)> {
        let t = subsystem_info::table
            .inner_join(subsysteminfo_componentinfo::table.inner_join(component_info::table))
            .filter(component_info::id.eq(comid));
        let mut q = t
            .select(subsystem_info::all_columns)
            .limit(query.size)
            .offset((query.page - 1) * query.size)
            .into_boxed();
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
        self.pool
            .get()?
            .transaction(|| Ok((q.load(&self.pool.get()?)?, cq.first(&self.pool.get()?)?)))
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
        Ok(q.first(&self.pool.get()?)?)
    }

    fn is_exist(&self, id: i32) -> dao::Result<bool> {
        Ok(subsystem_info::table
            .filter(subsystem_info::id.eq(id))
            .count()
            .execute(&self.pool.get()?)?
            > 0)
    }
}

pub struct ComponentInfoRepository(Pool<ConnectionManager<MysqlConnection>>);

impl ComponentInfoRepository {
    pub fn new(pool: Pool<ConnectionManager<MysqlConnection>>) -> Self {
        ComponentInfoRepository(pool)
    }
}

impl Deref for ComponentInfoRepository {
    type Target = Pool<ConnectionManager<MysqlConnection>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ComponentInfoStorer for ComponentInfoRepository {
    fn insert(&self, info: ComponentInfoInsert) -> dao::Result<usize> {
        Ok(diesel::insert_into(component_info::table)
            .values(info)
            .execute(&self.0.get()?)?)
    }

    fn bulk_insert(&self, infos: &Vec<ComponentInfoInsert>) -> dao::Result<usize> {
        Ok(diesel::insert_into(component_info::table)
            .values(infos)
            .execute(&self.0.get()?)?)
    }

    fn delete(&self, id: i32) -> dao::Result<usize> {
        Ok(diesel::delete(component_info::table.find(id)).execute(&self.0.get()?)?)
    }

    fn update(&self, id: i32, upd: ComponentInfoUpdate) -> dao::Result<usize> {
        Ok(diesel::update(component_info::table.find(id))
            .set(upd)
            .execute(&self.0.get()?)?)
    }

    fn get(&self, id: i32) -> dao::Result<ComponentInfo> {
        Ok(device_info::table.find(id).first(&self.0.get()?)?)
    }

    fn query(&self, query: ComponentInfoQuery) -> dao::Result<(Vec<ComponentInfo>, i64)> {
        let mut q = component_info::table
            .limit(query.size)
            .offset((query.page - 1) * query.size)
            .into_boxed();
        let mut cq = component_info::table.count().into_boxed();
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
        self.0
            .get()?
            .transaction(|| Ok((q.load(&self.0.get()?)?, cq.first(&self.0.get()?)?)))
    }

    fn query_by_subsystem_info(
        &self,
        subinfoid: i32,
        query: ComponentInfoQuery,
    ) -> dao::Result<(Vec<ComponentInfo>, i64)> {
        let t = component_info::table
            .inner_join(subsysteminfo_componentinfo::table.inner_join(subsystem_info::table))
            .filter(subsystem_info::id.eq(subinfoid));
        let mut q = t
            .select(component_info::all_columns)
            .limit(query.size)
            .offset((query.page - 1) * query.size)
            .into_boxed();
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
        self.0
            .get()?
            .transaction(|| Ok((q.load(&self.0.get()?)?, cq.first(&self.0.get()?)?)))
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
        Ok(q.first(&self.0.get()?)?)
    }

    fn is_exist(&self, id: i32) -> dao::Result<bool> {
        Ok(component_info::table
            .filter(component_info::id.eq(id))
            .count()
            .execute(&self.0.get()?)?
            > 0)
    }
}
