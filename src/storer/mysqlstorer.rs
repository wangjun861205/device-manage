use diesel::{ MysqlConnection, RunQueryDsl, QueryDsl, ExpressionMethods, TextExpressionMethods };
use super::super::model::*;
use super::super::schema::*;
use diesel::Connection;
use super::super::dao::DeviceInfoDao;
use diesel::r2d2::{ Pool, ConnectionManager};
use std::convert::From;
use std::fmt::{self, Display, Formatter};
use r2d2;
use super::super::dao;


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
    pool: Pool<ConnectionManager<MysqlConnection>>
}

impl DeviceInfoRepository {
    pub fn new(pool: Pool<ConnectionManager<MysqlConnection>>) -> Self {
        DeviceInfoRepository{pool}
    }
}

impl DeviceInfoDao for DeviceInfoRepository {
    fn insert_device_info(&self, info: DeviceInfoInsert) -> dao::Result<usize> {
        Ok(diesel::insert_into(device_info::table)
            .values(info)
            .execute(&self.pool.get()?)?)
    }

    fn bulk_insert_device_info(&self, infos: &Vec<DeviceInfoInsert>) -> dao::Result<usize> {
        Ok(diesel::insert_into(device_info::table)
            .values(infos)
            .execute(&self.pool.get()?)?)
    }

    fn delete_device_info(&self, id: i32) -> dao::Result<usize> {
        Ok(diesel::delete(device_info::table.find(id)).execute(&self.pool.get()?)?)
    }

    fn delete_device_infos(&self, query: DeviceInfoQuery) -> dao::Result<usize> {
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

    fn update_device_info( &self, id: i32, upd: DeviceInfoUpdate) -> dao::Result<usize> {
        Ok(diesel::update(device_info::table)
            .filter(device_info::id.eq(id))
            .set(upd)
            .execute(&self.pool.get()?)?)
    }

    fn get_device_info(&self, id: i32) -> dao::Result<DeviceInfo> {
        Ok(device_info::table.find(id).first(&self.pool.get()?)?)
    }

    fn query_device_infos(
        &self,
        query: DeviceInfoQuery,
    ) -> dao::Result<(Vec<DeviceInfo>, i64)> {
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
        self.pool.get()?.transaction(|| Ok((q.load(&self.pool.get()?)?, cq.first(&self.pool.get()?)?)))
    }

    fn query_device_infos_by_subsystem_info( &self, subinfoid: i32, query: DeviceInfoQuery) -> dao::Result<(Vec<DeviceInfo>, i64)> {
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
        self.pool.get()?.transaction(|| Ok((q.load(&self.pool.get()?)?, cq.first(&self.pool.get()?)?)))
    }

    fn count_device_info(&self, query: DeviceInfoQuery) -> dao::Result<i64> {
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
}
