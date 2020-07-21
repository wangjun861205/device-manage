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
pub fn add_device_info(conn: MysqlConn, info: Json<model::DeviceInfoInsert>) -> Result<String> {
    match diesel::insert_into(device_info::table)
        .values(info.0)
        .execute(&*conn)
    {
        Ok(_) => Ok(Json("success".to_owned())),
        Err(e) => Err(DaoError::new(&format!("新建设备信息失败: {}", e))),
    }
}

#[post("/device_infos", format="application/json", data="<infos>", rank = 1)]
pub fn bulk_add_device_info(conn: MysqlConn, infos: Json<Vec<model::DeviceInfoInsert>>) -> Result<usize> {
      conn.transaction(||{
            for info in infos.0.into_iter() {
                let res = diesel::insert_into(device_info::table).values(&info).execute(&*conn);
                if res.is_err() {
                    return res
                }
            }
            Ok(1)
      }).map_or_else(|e| Err(DaoError::new(&format!("{}", e))), |t| Ok(Json(t)))
}

#[get("/device_info/<id>")]
pub fn get_device_info(conn: MysqlConn, id: i32) -> Result<model::DeviceInfo> {
    let dev = device_info::table
        .find(id)
        .first::<model::DeviceInfo>(&*conn);
    match dev {
        Ok(d) => Ok(Json(d)),
        Err(e) => Err(DaoError::new(&format!("获取设备信息失败: {}", e))),
    }
}

#[get("/device_info?<name>&<model>&<maintain_interval>&<page>&<size>")]
pub fn query_device_info(
    conn: MysqlConn,
    name: Option<String>,
    model: Option<String>,
    maintain_interval: Option<i32>,
    page: i64,
    size: i64,
) -> Result<Vec<model::DeviceInfo>> {
    let mut query = device_info::table
        .limit(size)
        .offset((page - 1) * size)
        .into_boxed();
    if name.is_some() {
        query = query.filter(device_info::name.like(format!("%{}%", name.unwrap())));
    }
    if model.is_some() {
        query = query.filter(device_info::model.like(format!("%{}%", model.unwrap())));
    }
    if maintain_interval.is_some() {
        query = query.filter(device_info::maintain_interval.eq(maintain_interval.unwrap()));
    }
    let dev_infos: std::result::Result<Vec<model::DeviceInfo>, _> = query.load(&*conn);
    match dev_infos {
        Ok(l) => Ok(Json(l)),
        Err(e) => Err(DaoError::new(&format!("查询设备信息失败: {}", e))),
    }
}

#[delete("/device_info/<id>")]
pub fn delete_device_info(conn: MysqlConn, id: i32) -> Result<usize> {
    diesel::delete(device_info::table.filter(device_info::id.eq(id)))
    .execute(&*conn)
    .map_or_else(|e| Err(DaoError::new(&format!("{}", e))), |r| Ok(Json(r)))
}

#[put("/device_info/<id>", format="application/json", data="<req>")]
pub fn update_device_info(conn: MysqlConn, id: i32, req: Json<model::DeviceInfoUpdate>) -> Result<usize> {
    diesel::update(device_info::table.filter(device_info::id.eq(id)))
    .set(&req.0)
    .execute(&*conn)
    .map_or_else(|e| Err(DaoError::new(&format!("{}", e))), |r| Ok(Json(r)))
}

#[post("/device", format = "application/json", data = "<dev>")]
pub fn add_device(conn: MysqlConn, dev: Json<model::DeviceInsert>) -> Result<String> {
    match diesel::insert_into(device::table)
        .values(dev.0)
        .execute(&*conn)
    {
        Ok(_) => Ok(Json("success".into())),
        Err(e) => Err(DaoError::new(&format!("新建设备失败: {}", e))),
    }
}

#[post("/devices", format="application/json", data="<devs>")]
pub fn bulk_add_device(conn: MysqlConn, devs: Json<Vec<model::DeviceInsert>>) -> Result<usize> {
    conn.transaction(|| {
        for dev in devs.0.into_iter() {
            let res = diesel::insert_into(device::table).values(&dev).execute(&*conn);
            if res.is_err() {
                return res
            }
        }
        Ok(1)
    }).map_or_else(|e| Err(DaoError::new(&format!("{}", e))), |t| Ok(Json(t)))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceResp {
    pub device: model::Device,
    pub device_info: model::DeviceInfo,
}

#[get("/device/<id>")]
pub fn get_device(conn: MysqlConn, id: i32) -> Result<DeviceResp> {
    let dev = device::table.find(id).first::<model::Device>(&*conn);
    match dev {
        Ok(d) => {
            let info = device_info::table.find(d.device_info_id).first(&*conn);
            match info {
                Ok(i) => { 
                    Ok(Json(DeviceResp{
                        device: d,
                        device_info: i,
                    }))
                },
                Err(e) => {
                    Err(DaoError::new(&format!("获取设备信息失败: {}", e)))
                }
            }
        },
        Err(e) => {
            Err(DaoError::new(&format!("获取设备失败: {}", e)))
        }
    }
}

#[get("/device?<query..>")]
pub fn query_device(conn: MysqlConn, query: model::DeviceQuery) -> Result<Vec<DeviceResp>> {
    let mut q = device_info::table.inner_join(device::table).into_boxed();
    if query.name.is_some() {
        q = q.filter(device_info::name.like(format!("%{}%", query.name.unwrap())));
    }
    if query.model.is_some() {
        q = q.filter(device_info::model.like(format!("%{}%", query.model.unwrap())));
    }
    if query.unicode.is_some() {
        q = q.filter(device::unicode.like(format!("%{}%", query.unicode.unwrap())));
    }
    if query.last_start_at.is_some() {
        q = q.filter(device::last_start_at.eq(query.last_start_at.unwrap()));
    }
    if query.last_stop_at.is_some() {
        q = q.filter(device::last_stop_at.eq(query.last_stop_at.unwrap()))
    }
    if query.total_duration.is_some() {
        q = q.filter(device::total_duration.eq(query.total_duration.unwrap()));
    }
    if query.status.is_some() {
        q = q.filter(device::status.eq(query.status.unwrap()));
    }
    if query.page.is_some() && query.size.is_some() {
        q = q.limit(query.size.unwrap()).offset((query.page.unwrap()-1)*query.size.unwrap())
    }
    let l = q.load::<(model::DeviceInfo, model::Device)>(&*conn).unwrap();
    Ok(Json(l.into_iter().map(|t| {
        DeviceResp{
            device_info: t.0,
            device: t.1,
        }
    }).collect()))
}

#[put("/device/<id>", format="application/json", data="<req>")]
pub fn update_device(conn: MysqlConn, id: i32, req: Json<model::DeviceUpdate>) -> Result<usize> {
    diesel::update(device::table.filter(device::id.eq(id)))
    .set(&req.0)
    .execute(&*conn)
    .map_or_else(|e| Err(DaoError::new(&format!("{}", e))), |r| Ok(Json(r)))
}

#[delete("/device/<id>")]
pub fn delete_device(conn: MysqlConn, id: i32) -> Result<usize> {
    diesel::delete(device::table.filter(device::id.eq(id)))
    .execute(&*conn)
    .map_or_else(|e| Err(DaoError::new(&format!("{}", e))), |t| Ok(Json(t)))
}

#[post("/subsystem_info", format="application/json", data="<info>")]
pub fn add_subsystem_info(conn: MysqlConn, info: Json<model::SubsystemInfoInsert>) -> Result<usize> {
    diesel::insert_into(subsystem_info::table)
    .values(info.0).execute(&*conn)
    .map_or_else(|e| Err(DaoError::from(e)), |t| Ok(Json(t)))
}

#[post("/subsystem_infos", format="application/json", data="<infos>")]
pub fn bulk_add_subsystem_info(conn: MysqlConn, infos: Json<Vec<model::SubsystemInfoInsert>>) -> Result<usize> {
    conn.transaction(|| {
        for info in infos.0.into_iter() {
            diesel::insert_into(subsystem_info::table).values(info).execute(&*conn)?;
        }
        Ok(1)
    }).map_or_else(|e: diesel::result::Error| Err(DaoError::from(e)), |t| Ok(Json(t as usize)))
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
pub struct DeviceGroupResp(model::Device, model::DeviceInfo);


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct SubsystemResp(model::Subsystem, model::SubsystemInfo, DeviceGroupResp);


#[get("/subsystems?<query..>")]
pub fn query_subsystem(conn: MysqlConn, query: Form<model::SubsystemQuery>) -> Result<Vec<SubsystemResp>> {
    let mut q = subsystem::table.inner_join(subsystem_info::table).inner_join(device::table.inner_join(device_info::table)).into_boxed();
    if let Some(v) = query.0.device_name { 
        q = q.filter(device_info::name.eq(v));
    }
    if let Some(v) = query.0.device_model { 
        q = q.filter(device_info::model.eq(v)); 
    }
    if let Some(v) =  query.0.device_maintain_interval {
        q = q.filter(device_info::maintain_interval.eq(v));
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
        q = q.filter(subsystem_info::name.eq(v))
    }
    if let Some(v) = query.0.subsystem_maintain_interval {
        q = q.filter(subsystem_info::maintain_interval.eq(v));
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
    diesel::delete(subsystem::table)
    .filter(subsystem::id.eq(id)).execute(&*conn)
    .map_or_else(|e| Err(DaoError::from(e)), |t| Ok(Json(t)))
}


