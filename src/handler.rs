use super::dao::{DaoError, Result};
use super::diesel::QueryDsl;
use super::model;
use super::schema::*;
use super::MysqlConn;
use diesel;
use diesel::connection::*;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use diesel::TextExpressionMethods;
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
use rocket::request::Form;

#[post("/device_info", format = "application/json", data = "<info>")]
pub fn add_device_info(conn: MysqlConn, info: Json<model::DeviceInfoInsert>) -> Result<usize> {
    Ok(Json(diesel::insert_into(device_info::table)
    .values(info.0)
    .execute(&*conn)?))
}

#[post("/device_infos", format="application/json", data="<infos>", rank = 1)]
pub fn bulk_add_device_info(conn: MysqlConn, infos: Json<Vec<model::DeviceInfoInsert>>) -> Result<usize> {
      Ok(Json(conn.transaction::<_, diesel::result::Error, _>(||{
            for info in infos.0.into_iter() {
                diesel::insert_into(device_info::table).values(&info).execute(&*conn)?;
            }
            Ok(1 as usize)
      })?))
}

#[get("/device_info/<id>")]
pub fn get_device_info(conn: MysqlConn, id: i32) -> Result<model::DeviceInfo> {
    Ok(
        Json(
            device_info::table
        .find(id)
        .first::<model::DeviceInfo>(&*conn)?))
}

#[get("/device_info?<query..>")]
pub fn query_device_info(conn: MysqlConn, query: Form<model::DeviceInfoQuery>) -> Result<Vec<model::DeviceInfo>> {
    let mut q = device_info::table.into_boxed();
    if let Some(v) = query.0.name {
        q = q.filter(device_info::name.like(format!("%{}%", v)));
    }
    if let Some(v) = query.0.model {
        q = q.filter(device_info::model.like(format!("%{}%", v)));
    }
    if let Some(v) = query.0.maintain_interval {
        q = q.filter(device_info::maintain_interval.eq(v));
    }
    if let Some(pv) = query.0.page {
        if let Some(sv) = query.0.size {
            q = q.limit(sv).offset((pv-1)*sv)
        }
    }
    Ok(Json(q.load(&*conn)?))
}


#[delete("/device_info/<id>")]
pub fn delete_device_info(conn: MysqlConn, id: i32) -> Result<usize> {
    Ok(Json(diesel::delete(device_info::table.filter(device_info::id.eq(id))).execute(&*conn)?))
}

#[put("/device_info/<id>", format="application/json", data="<req>")]
pub fn update_device_info(conn: MysqlConn, id: i32, req: Json<model::DeviceInfoUpdate>) -> Result<usize> {
    Ok(
        Json(
            diesel::update(device_info::table.filter(device_info::id.eq(id)))
            .set(&req.0)
            .execute(&*conn)?
       )
    )
}

#[post("/device", format = "application/json", data = "<dev>")]
pub fn add_device(conn: MysqlConn, dev: Json<model::DeviceInsert>) -> Result<usize> {
    Ok(Json(diesel::insert_into(device::table)
        .values(dev.0)
        .execute(&*conn)?))
}

#[post("/devices", format="application/json", data="<devs>")]
pub fn bulk_add_device(conn: MysqlConn, devs: Json<Vec<model::DeviceInsert>>) -> Result<usize> {
    Ok(Json(conn.transaction::<_, diesel::result::Error, _>(|| {
        for dev in devs.0.into_iter() {
            diesel::insert_into(device::table).values(&dev).execute(&*conn)?;
        }
        Ok(1)
    })?))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceResp {
    pub device: model::Device,
    pub device_info: model::DeviceInfo,
}

#[get("/device/<id>")]
pub fn get_device(conn: MysqlConn, id: i32) -> Result<model::Device> {
    Ok(Json(device::table.find(id).first::<model::Device>(&*conn)?))
}


#[get("/device?<query..>")]
pub fn query_device(conn: MysqlConn, query: Form<model::DeviceQuery>) -> Result<Vec<model::Device>> {
    let mut q = device::table.into_boxed();
    if let Some(v) = query.0.name {
        q = q.filter(device::name.like(format!("%{}%", v)));
    }
    if let Some(v) = query.0.model {
        q = q.filter(device::model.like(format!("%{}%", v)));
    }
    if let Some(v) = query.0.maintain_interval {
        q = q.filter(device::maintain_interval.eq(v));
    }
    if let Some(v) = query.0.unicode {
        q = q.filter(device::unicode.like(format!("%{}%", v)));
    }
    if let Some(v) = query.0.last_start_at_begin {
        q = q.filter(device::last_start_at.ge(v.0));
    }
    if let Some(v) = query.0.last_start_at_end {
        q = q.filter(device::last_start_at.lt(v.0));
    }
    if let Some(v) = query.0.last_stop_at_begin {
        q = q.filter(device::last_stop_at.ge(v.0))
    }
    if let Some(v) = query.0.total_duration_begin {
        q = q.filter(device::total_duration.ge(v));
    }
    if let Some(v) = query.0.total_duration_end {
        q = q.filter(device::total_duration.lt(v))
    }
    if let Some(v) = query.0.status {
        q = q.filter(device::status.eq(v));
    }
    if let Some(pv) = query.0.page {
        if let Some(sv) = query.0.size {
            q = q.limit(sv).offset((pv-1)*sv)
        }
    }
    Ok(Json(q.load(&*conn)?))
}

#[put("/device/<id>", format="application/json", data="<req>")]
pub fn update_device(conn: MysqlConn, id: i32, req: Json<model::DeviceUpdate>) -> Result<usize> {
     Ok(Json(diesel::update(device::table.filter(device::id.eq(id)))
    .set(&req.0)
    .execute(&*conn)?))
}

#[delete("/device/<id>")]
pub fn delete_device(conn: MysqlConn, id: i32) -> Result<usize> {
    Ok(Json(diesel::delete(device::table.filter(device::id.eq(id)))
    .execute(&*conn)?))
}

#[post("/subsystem_info", format="application/json", data="<info>")]
pub fn add_subsystem_info(conn: MysqlConn, info: Json<model::SubsystemInfoInsert>) -> Result<usize> {
    Ok(Json(diesel::insert_into(subsystem_info::table)
    .values(info.0).execute(&*conn)?))
}

#[post("/subsystem_infos", format="application/json", data="<infos>")]
pub fn bulk_add_subsystem_info(conn: MysqlConn, infos: Json<Vec<model::SubsystemInfoInsert>>) -> Result<usize> {
    Ok(Json(conn.transaction::<_, diesel::result::Error, _>(|| {
        for info in infos.0.into_iter() {
            diesel::insert_into(subsystem_info::table).values(info).execute(&*conn)?;
        }
        Ok(1)
    })?))
}

#[get("/subsysetem_info/<id>")]
pub fn get_subsystem_info(conn: MysqlConn, id: i32) -> Result<model::SubsystemInfo> {
    Ok(Json(subsystem_info::table.filter(subsystem_info::id.eq(id)).first(&*conn)?))
}

#[get("/subsystem_infos?<query..>")]
pub fn query_subsystem_info(conn: MysqlConn, query: Form<model::SubsystemInfoQuery>) -> Result<Vec<model::SubsystemInfo>> {
    let mut q = subsystem_info::table.into_boxed();
    if let Some(v) = query.0.name {
        q = q.filter(subsystem_info::name.eq(v))
    }
    if let Some(v) = query.0.maintain_interval {
        q = q.filter(subsystem_info::maintain_interval.eq(v))
    }
    if let Some(pv) = query.0.page {
        if let Some(sv) = query.0.size {
            q = q.limit(sv).offset((pv-1)*sv)
        }
    }
    Ok(Json(q.load(&*conn)?))
}

#[delete("/subsystem_info/<id>")]
pub fn delete_subsystem_info(conn: MysqlConn, id: i32) -> Result<usize> {
    Ok(Json(diesel::delete(subsystem_info::table).filter(subsystem_info::id.eq(id)).execute(&*conn)?))
}


#[post("/subsystem", format="application/json", data="<sys>")]
pub fn add_subsystem(conn: MysqlConn, sys: Json<model::SubsystemInsert>) -> Result<usize> {
    diesel::insert_into(subsystem::table)
    .values(sys.0)
    .execute(&*conn)
    .map_or_else(|e| Err(DaoError::new(&format!("{}", e))), |t| Ok(Json(t)))
}

#[post("/subsystems", format="application/json", data="<ss>")]
pub fn bulk_add_subsystem(conn: MysqlConn, ss: Json<Vec<model::SubsystemInsert>>) -> Result<usize> {
    conn.0.transaction(||{
        for s in ss.0.into_iter() {
            match diesel::insert_into(subsystem::table).values(s).execute(&*conn) {
                Err(e) => return Err(e),
                Ok(_) => continue,
            }
        }
        Ok(Json(1 as usize))
    }).map_or_else(|e| Err(DaoError::from(e)), |t| Ok(t))
}

#[get("/subsystem/<id>")]
pub fn get_subsystem(conn: MysqlConn, id: i32) -> Result<model::Subsystem> {
    subsystem::table.filter(subsystem::id.eq(id)).first(&*conn)
    .map_or_else(|e| Err(DaoError::from(e)), |t| Ok(Json(t)))
}


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct SubsystemResp(model::Subsystem, model::Device);


#[get("/subsystems?<query..>")]
pub fn query_subsystem(conn: MysqlConn, query: Form<model::SubsystemQuery>) -> Result<Vec<SubsystemResp>> {
    let mut q = subsystem::table.inner_join(device::table).into_boxed();
    if let Some(v) = query.0.device_name { 
        q = q.filter(device::name.eq(v));
    }
    if let Some(v) = query.0.device_model { 
        q = q.filter(device::model.eq(v)); 
    }
    if let Some(v) =  query.0.device_maintain_interval {
        q = q.filter(device::maintain_interval.eq(v));
    }
    if let Some(v) = query.0.device_unicode{
        q = q.filter(device::unicode.eq(v));
    }
    if let Some(v) = query.0.device_last_start_at {
        q = q.filter(device::last_start_at.eq(v.0));
    }
    if let Some(v) = query.0.device_last_stop_at {
        q = q.filter(device::last_stop_at.eq(v.0));
    }
    if let Some(v) = query.0.device_total_duration {
        q = q.filter(device::total_duration.eq(v));
    }
    if let Some(v) = query.0.device_status {
        q = q.filter(device::status.eq(v));
    }
    if let Some(v) = query.0.subsystem_name {
        q = q.filter(subsystem::name.eq(v))
    }
    if let Some(v) = query.0.subsystem_maintain_interval {
        q = q.filter(subsystem::maintain_interval.eq(v));
    }
    if let Some(pv) = query.0.page {
        if let Some(sv) = query.0.size {
            q = q.limit(sv).offset((pv-1)*sv)
        }
    }
    q.load(&*conn).map_or_else(|e| Err(DaoError::from(e)), |t| Ok(Json(t)))
}

#[delete("/subsystem/<id>")]
pub fn update_subsystem(conn: MysqlConn, id: i32) -> Result<usize> {
    Ok(Json(diesel::delete(subsystem::table)
    .filter(subsystem::id.eq(id)).execute(&*conn)?))
}


