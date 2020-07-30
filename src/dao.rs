use super::model::*;
use super::schema::*;
use diesel::mysql::MysqlConnection;
use diesel::result::QueryResult;
use diesel::Connection;
use diesel::{
    BelongingToDsl, BoolExpressionMethods, ExpressionMethods, GroupedBy, QueryDsl, RunQueryDsl,
    TextExpressionMethods,
};

#[cfg(test)]
mod tests {
    use super::super::diesel;
    use super::*;
    fn new_conn() -> MysqlConnection {
        diesel::MysqlConnection::establish("mysql://wangjun:Wt20110523@localhost/devicemanage")
            .expect("failed to connect to mysql")
    }
    #[test]
    fn test_delete_device_infos() {
        let conn = new_conn();
        delete_device_infos(
            &conn,
            DeviceInfoQuery {
                ..Default::default()
            },
        )
        .expect("failed to delete device_infos");
    }
    #[test]
    fn test_insert_device_info() {
        test_delete_device_infos();
        let conn = new_conn();
        insert_device_info(
            &conn,
            DeviceInfoInsert {
                name: "test name".into(),
                model: "test model".into(),
                maintain_interval: Some(30),
            },
        )
        .expect("failed to insert device_info");
        let c = count_device_info(
            &conn,
            DeviceInfoQuery {
                ..Default::default()
            },
        )
        .unwrap();
        assert!(c == 1, "c = {}", c)
    }

    #[test]
    fn test_bulk_insert_device_info() {
        test_delete_device_infos();
        let mut l = Vec::new();
        for i in 1..=100 {
            l.push(DeviceInfoInsert {
                name: format!("name {}", i),
                model: format!("model {}", i),
                maintain_interval: Some(i),
            })
        }
        let conn = new_conn();
        bulk_insert_device_info(&conn, &l).expect("failed to bulk insert device_infos");
        let count = count_device_info(
            &conn,
            DeviceInfoQuery {
                ..Default::default()
            },
        )
        .expect("failed to count device_info");
        assert!(count == 100, "count = {}", count);
    }

    #[test]
    fn test_update_device_info() {
        test_delete_device_infos();
        let conn = new_conn();
        insert_device_info(
            &conn,
            DeviceInfoInsert {
                name: "test name".into(),
                model: "test model".into(),
                maintain_interval: Some(10),
            },
        )
        .unwrap();
        let devinfo = &query_device_infos(
            &conn,
            DeviceInfoQuery {
                name: Some("test name".into()),
                page: 1,
                size: 1,
                ..Default::default()
            },
        )
        .unwrap()
        .0[0];
        update_device_info(
            &conn,
            devinfo.id,
            DeviceInfoUpdate {
                maintain_interval: Some(100),
                ..Default::default()
            },
        )
        .unwrap();
        let devinfo = &query_device_infos(
            &conn,
            DeviceInfoQuery {
                name: Some("test name".into()),
                page: 1,
                size: 1,
                ..Default::default()
            },
        )
        .unwrap()
        .0[0];
        assert!(
            devinfo.maintain_interval == 100,
            "maintain_interval is {}",
            devinfo.maintain_interval
        )
    }
}

// ==============================================trait===============================================================


#[derive(Debug)]
pub struct Error(pub String);

use std::fmt::{ self, Display, Formatter };

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", &self)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

// use super::storer::mysqlstorer::Result;
pub trait DeviceInfoDao {
    fn insert_device_info(&self, info: DeviceInfoInsert) -> Result<usize>;
    fn bulk_insert_device_info(&self, infos: &Vec<DeviceInfoInsert>,) -> Result<usize>;
    fn delete_device_info(&self, id: i32) -> Result<usize>;
    fn delete_device_infos(&self, query: DeviceInfoQuery) -> Result<usize>;
    fn update_device_info(&self, id: i32, upd: DeviceInfoUpdate) -> Result<usize>;
    fn get_device_info(&self, id: i32) -> Result<DeviceInfo>;
    fn query_device_infos(&self, query: DeviceInfoQuery) -> Result<(Vec<DeviceInfo>, i64)>;
    fn query_device_infos_by_subsystem_info(&self, subinfoid: i32, query: DeviceInfoQuery) -> Result<(Vec<DeviceInfo>, i64)>;
    fn count_device_info(&self, query: DeviceInfoQuery) -> Result<i64>;
}

// ==============================================device_info=========================================================

pub fn insert_device_info(conn: &MysqlConnection, info: DeviceInfoInsert) -> QueryResult<usize> {
    Ok(diesel::insert_into(device_info::table)
        .values(info)
        .execute(conn)?)
}

pub fn bulk_insert_device_info(conn: &MysqlConnection, infos: &Vec<DeviceInfoInsert>) -> QueryResult<usize> {
    Ok(diesel::insert_into(device_info::table)
        .values(infos)
        .execute(conn)?)
}

pub fn delete_device_info(conn: &MysqlConnection, id: i32) -> QueryResult<usize> {
    Ok(diesel::delete(device_info::table.find(id)).execute(conn)?)
}

pub fn delete_device_infos(conn: &MysqlConnection, query: DeviceInfoQuery) -> QueryResult<usize> {
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
    Ok(q.execute(conn)?)
}

pub fn update_device_info( conn: &MysqlConnection, id: i32, upd: DeviceInfoUpdate) -> QueryResult<usize> {
    Ok(diesel::update(device_info::table)
        .filter(device_info::id.eq(id))
        .set(upd)
        .execute(conn)?)
}

pub fn get_device_info(conn: &MysqlConnection, id: i32) -> QueryResult<DeviceInfo> {
    Ok(device_info::table.find(id).first(conn)?)
}

pub fn query_device_infos(
    conn: &MysqlConnection,
    query: DeviceInfoQuery,
) -> QueryResult<(Vec<DeviceInfo>, i64)> {
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
    conn.transaction(|| Ok((q.load(conn)?, cq.first(conn)?)))
}

pub fn query_device_infos_by_subsystem_info( conn: &MysqlConnection, subinfoid: i32, query: DeviceInfoQuery) -> QueryResult<(Vec<DeviceInfo>, i64)> {
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
    conn.transaction(|| Ok((q.load(conn)?, cq.first(conn)?)))
}

pub fn count_device_info(conn: &MysqlConnection, query: DeviceInfoQuery) -> QueryResult<i64> {
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
    Ok(q.count().first(conn)?)
}

// ===================================================subsystem_info======================================================

pub fn insert_subsystem_info(
    conn: &MysqlConnection,
    info: SubsystemInfoInsert,
) -> QueryResult<usize> {
    Ok(diesel::insert_into(subsystem_info::table)
        .values(info)
        .execute(conn)?)
}

pub fn bulk_insert_subsystem_info(
    conn: &MysqlConnection,
    infos: &Vec<SubsystemInfoInsert>,
) -> QueryResult<usize> {
    Ok(diesel::insert_into(subsystem_info::table)
        .values(infos)
        .execute(conn)?)
}

pub fn delete_subsystem_info(conn: &MysqlConnection, id: i32) -> QueryResult<usize> {
    Ok(diesel::delete(subsystem_info::table.find(id)).execute(conn)?)
}

pub fn update_subsystem_info(
    conn: &MysqlConnection,
    id: i32,
    upd: SubsystemInfoUpdate,
) -> QueryResult<usize> {
    Ok(diesel::update(subsystem_info::table.find(id))
        .set(upd)
        .execute(conn)?)
}

pub fn get_subsystem_info(
    conn: &MysqlConnection,
    id: i32,
) -> QueryResult<(SubsystemInfo, Vec<DeviceInfo>, Vec<ComponentInfo>)> {
    let subinfo: SubsystemInfo = subsystem_info::table.find(id).first(conn)?;
    let devinfos: Vec<DeviceInfo> = DeviceinfoSubsysteminfo::belonging_to(&subinfo)
        .inner_join(device_info::table)
        .select(device_info::all_columns)
        .load(conn)?;
    let cominfos: Vec<ComponentInfo> = SubsysteminfoComponentinfo::belonging_to(&subinfo)
        .inner_join(component_info::table)
        .select(component_info::all_columns)
        .load(conn)?;
    Ok((subinfo, devinfos, cominfos))
}

pub fn query_subsystem_infos(
    conn: &MysqlConnection,
    query: SubsystemInfoQuery,
) -> QueryResult<(Vec<SubsystemInfo>, i64)> {
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
    conn.transaction(|| Ok((q.load(conn)?, cq.first(conn)?)))
}

pub fn query_subsystem_infos_by_device_info(
    conn: &MysqlConnection,
    devinfoid: i32,
    query: SubsystemInfoQuery,
) -> QueryResult<(Vec<SubsystemInfo>, i64)> {
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
    conn.transaction(|| Ok((q.load(conn)?, cq.first(conn)?)))
}

pub fn query_subsystem_infos_by_component_info(
    conn: &MysqlConnection,
    comid: i32,
    query: SubsystemInfoQuery,
) -> QueryResult<(Vec<SubsystemInfo>, i64)> {
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
    conn.transaction(|| Ok((q.load(conn)?, cq.first(conn)?)))
}

pub fn count_subsystem_info(conn: &MysqlConnection, query: SubsystemInfoQuery) -> QueryResult<i64> {
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
    Ok(q.first(conn)?)
}

// =======================================================component_info===================================================

pub fn insert_component_info(
    conn: &MysqlConnection,
    info: ComponentInfoInsert,
) -> QueryResult<usize> {
    Ok(diesel::insert_into(component_info::table)
        .values(info)
        .execute(conn)?)
}

pub fn bulk_insert_component_info(
    conn: &MysqlConnection,
    infos: &Vec<ComponentInfoInsert>,
) -> QueryResult<usize> {
    Ok(diesel::insert_into(component_info::table)
        .values(infos)
        .execute(conn)?)
}

pub fn delete_component_info(conn: &MysqlConnection, id: i32) -> QueryResult<usize> {
    Ok(diesel::delete(component_info::table.find(id)).execute(conn)?)
}

pub fn update_component_info(
    conn: &MysqlConnection,
    id: i32,
    upd: ComponentInfoUpdate,
) -> QueryResult<usize> {
    Ok(diesel::update(component_info::table.find(id))
        .set(upd)
        .execute(conn)?)
}

pub fn get_component_info(
    conn: &MysqlConnection,
    id: i32,
) -> QueryResult<(DeviceInfo, SubsystemInfo, ComponentInfo)> {
    Ok(device_info::table
        .inner_join(
            deviceinfo_subsysteminfo::table.inner_join(
                subsystem_info::table.inner_join(
                    subsysteminfo_componentinfo::table.inner_join(component_info::table),
                ),
            ),
        )
        .filter(component_info::id.eq(id))
        .select((
            device_info::all_columns,
            subsystem_info::all_columns,
            component_info::all_columns,
        ))
        .first(conn)?)
}

pub fn query_component_infos(
    conn: &MysqlConnection,
    query: ComponentInfoQuery,
) -> QueryResult<(Vec<ComponentInfo>, i64)> {
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
    conn.transaction(|| Ok((q.load(conn)?, cq.first(conn)?)))
}

pub fn query_component_infos_by_subsystem_info(
    conn: &MysqlConnection,
    subinfoid: i32,
    query: ComponentInfoQuery,
) -> QueryResult<(Vec<ComponentInfo>, i64)> {
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
    conn.transaction(|| Ok((q.load(conn)?, cq.first(conn)?)))
}

pub fn count_component_info(conn: &MysqlConnection, query: ComponentInfoQuery) -> QueryResult<i64> {
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
    Ok(q.first(conn)?)
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
