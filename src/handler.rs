use super::model::*;
use super::MysqlConn;
use super::dao;
use diesel;
use rocket_contrib::json::Json;
use rocket::request::Form;
use std::fmt::Display;
use std::convert::From;
use rocket::response::Responder;
use rocket::Request;
use rocket::response;
use std::io::Cursor;
use rocket::http::{ Status, Header };
use std::fmt;



#[derive(Debug)]
pub struct Error {
    status: Status,
    detail: String,
}

impl Error {
    pub fn new(status: Status, detail: String) -> Self {
        Self {
            status,
            detail,
        }
    }
}

impl Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.status, self.detail)
        }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Error {
        Self::new(Status::new(599, "database error"), format!("{}", e))
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        response::ResponseBuilder::new(response::Response::new())
        .status(self.status)
        .sized_body(Cursor::new(self.detail))
        .header(Header::new("Content-Type", "text/plain;charset=utf-8"))
        .ok()
    }
}


pub type Result<T> = std::result::Result<Json<T>, Error>;


#[post("/device_info", format = "application/json", data = "<info>")]
pub fn add_device_info(conn: MysqlConn, info: Json<DeviceInfoInsert>) -> Result<usize> {
    Ok(Json(dao::insert_device_info(&*conn, info.0)?))
}


#[post("/device_infos", format="application/json", data="<infos>", rank = 1)]
pub fn bulk_add_device_info(conn: MysqlConn, infos: Json<Vec<DeviceInfoInsert>>) -> Result<usize> {
    Ok(Json(dao::bulk_insert_device_info(&*conn, &infos.0)?))
}

#[delete("/device_info/<id>")]
pub fn delete_device_info(conn: MysqlConn, id: i32) -> Result<usize> {
    Ok(Json(dao::delete_device_info(&*conn, id)?))
}

#[put("/device_info/<id>", format="application/json", data="<req>")]
pub fn update_device_info(conn: MysqlConn, id: i32, req: Json<DeviceInfoUpdate>) -> Result<usize> {
    Ok(Json(dao::update_device_info(&*conn, id, req.0)?))
}

#[get("/device_info/<id>")]
pub fn get_device_info(conn: MysqlConn, id: i32) -> Result<DeviceInfo> {
    Ok(Json(dao::get_device_info(&*conn, id)?))
}

#[get("/device_info?<query..>")]
pub fn query_device_info(conn: MysqlConn, query: Form<DeviceInfoQuery>) -> Result<Vec<DeviceInfo>> {
    Ok(Json(dao::query_device_infos(&*conn, query.0)?))
}


#[post("/device", format = "application/json", data = "<dev>")]
pub fn add_device(conn: MysqlConn, dev: Json<DeviceInsert>) -> Result<usize> {
    Ok(Json(dao::insert_device(&*conn, dev.0)?))
}

#[post("/devices", format="application/json", data="<devs>")]
pub fn bulk_add_device(conn: MysqlConn, devs: Json<Vec<DeviceInsert>>) -> Result<usize> {
    Ok(Json(dao::bulk_insert_device(&*conn, &devs.0)?))
}

#[delete("/device/<id>")]
pub fn delete_device(conn: MysqlConn, id: i32) -> Result<usize> {
    Ok(Json(dao::delete_device(&*conn, id)?))
}

#[put("/device/<id>", format="application/json", data="<req>")]
pub fn update_device(conn: MysqlConn, id: i32, req: Json<DeviceUpdate>) -> Result<usize> {
    Ok(Json(dao::update_device(&*conn, id, req.0)?))
}

#[get("/device/<id>")]
pub fn get_device(conn: MysqlConn, id: i32) -> Result<(Device, Vec<(Subsystem, Vec<Component>)>)> {
    Ok(Json(dao::get_device(&*conn, id)?))
}

#[get("/device?<query..>")]
pub fn query_device(conn: MysqlConn, query: Form<DeviceQuery>) -> Result<Vec<(Device, Vec<(Subsystem, Vec<Component>)>)>> {
    Ok(Json(dao::query_device(&*conn, query.0)?))
}


#[post("/subsystem_info", format="application/json", data="<info>")]
pub fn add_subsystem_info(conn: MysqlConn, info: Json<SubsystemInfoInsert>) -> Result<usize> {
    Ok(Json(dao::insert_subsystem_info(&*conn, info.0)?))
}

#[post("/subsystem_infos", format="application/json", data="<infos>")]
pub fn bulk_add_subsystem_info(conn: MysqlConn, infos: Json<Vec<SubsystemInfoInsert>>) -> Result<usize> {
    Ok(Json(dao::bulk_insert_subsystem_info(&*conn, &infos.0)?))
}

#[delete("/subsystem_info/<id>")]
pub fn delete_subsystem_info(conn: MysqlConn, id: i32) -> Result<usize> {
    Ok(Json(dao::delete_subsystem_info(&*conn, id)?))
}

#[put("/subsystem_info/<id>", format="application/json", data="<upd>")]
pub fn update_subsystem_info(conn: MysqlConn, id: i32, upd: Json<SubsystemInfoUpdate>) -> Result<usize> {
    Ok(Json(dao::update_subsystem_info(&*conn, id, upd.0)?))
}

#[get("/subsysetem_info/<id>")]
pub fn get_subsystem_info(conn: MysqlConn, id: i32) -> Result<(SubsystemInfo, Vec<DeviceInfo>, Vec<ComponentInfo>)> {
    Ok(Json(dao::get_subsystem_info(&*conn, id)?))
}

#[get("/subsystem_infos?<query..>")]
pub fn query_subsystem_info(conn: MysqlConn, query: Form<SubsystemInfoQuery>) -> Result<Vec<(SubsystemInfo, Vec<DeviceInfo>, Vec<ComponentInfo>)>> {
    Ok(Json(dao::query_subsystem_info(&*conn, query.0)?))
}

#[post("/subsystem", format="application/json", data="<sys>")]
pub fn add_subsystem(conn: MysqlConn, sys: Json<SubsystemInsert>) -> Result<usize> {
    Ok(Json(dao::insert_subsystem(&*conn, sys.0)?))
}

#[post("/subsystems", format="application/json", data="<ss>")]
pub fn bulk_add_subsystem(conn: MysqlConn, ss: Json<Vec<SubsystemInsert>>) -> Result<usize> {
    Ok(Json(dao::bulk_insert_subsystem(&*conn, &ss.0)?))
}

#[delete("/subsystem/<id>")]
pub fn delete_subsystem(conn: MysqlConn, id: i32) -> Result<usize> {
    Ok(Json(dao::delete_subsystem(&*conn, id)?))
}

#[put("/subsystem/<id>", format="application/json", data="<upd>")]
pub fn update_subsystem(conn: MysqlConn, id: i32, upd: Json<SubsystemUpdate>) -> Result<usize> {
    Ok(Json(dao::udpate_subsystem(&*conn, id, upd.0)?))
}

#[get("/subsystem/<id>")]
pub fn get_subsystem(conn: MysqlConn, id: i32) -> Result<(Device, Subsystem, Vec<Component>)> {
    Ok(Json(dao::get_subsystem(&*conn, id)?))
}

#[get("/subsystems?<query..>")]
pub fn query_subsystem(conn: MysqlConn, query: Form<SubsystemQuery>) -> Result<Vec<(Device, Subsystem, Vec<Component>)>> {
    Ok(Json(dao::query_subsystem(&*conn, query.0)?))
}


#[post("/component_info", format="application/json", data="<info>")]
pub fn add_component_info(conn: MysqlConn, info: Json<ComponentInfoInsert>) -> Result<usize> {
    Ok(Json(dao::insert_component_info(&*conn, info.0)?))
}

#[post("/component_infos", format="application/json", data="<infos>")]
pub fn bulk_add_component_info(conn: MysqlConn, infos: Json<Vec<ComponentInfoInsert>>) -> Result<usize> {
    Ok(Json(dao::bulk_insert_component_info(&*conn, &infos.0)?))
}

#[delete("/component_info/<id>")]
pub fn delete_component_info(conn: MysqlConn, id: i32) -> Result<usize> {
    Ok(Json(dao::delete_component_info(&*conn, id)?))
}

#[put("/component_info/<id>", format="application/json", data="<upd>")]
pub fn update_component_info(conn: MysqlConn, id: i32, upd: Json<ComponentInfoUpdate>) -> Result<usize> {
    Ok(Json(dao::update_component_info(&*conn, id, upd.0)?))
}

#[get("/component_info/<id>")]
pub fn get_component_info(conn: MysqlConn, id: i32) -> Result<(DeviceInfo, SubsystemInfo, ComponentInfo)> {
    Ok(Json(dao::get_component_info(&*conn, id)?))
}

#[get("/component_info?<query..>")]
pub fn query_component_info(conn: MysqlConn, query: Form<ComponentInfoQuery>) -> Result<Vec<(DeviceInfo, SubsystemInfo, ComponentInfo)>> {
    Ok(Json(dao::query_component_info(&*conn, query.0)?))
}

#[post("/component", format="application/json", data="<com>")]
pub fn add_component(conn: MysqlConn, com: Json<ComponentInsert>) -> Result<usize> {
    Ok(Json(dao::insert_component(&*conn, com.0)?))
}

#[post("/components", format="application/json", data="<coms>")]
pub fn bulk_add_component(conn: MysqlConn, coms: Json<Vec<ComponentInsert>>) -> Result<usize> {
    Ok(Json(dao::bulk_insert_component(&*conn, &coms.0)?))
}

#[delete("/component/<id>")]
pub fn delete_component(conn: MysqlConn, id: i32) -> Result<usize> {
    Ok(Json(dao::delete_component(&*conn, id)?))
}

#[put("/component/<id>", format="application/json", data="<upd>")]
pub fn update_component(conn: MysqlConn, id: i32, upd: Json<ComponentUpdate>) -> Result<usize> {
    Ok(Json(dao::update_component(&*conn, id, upd.0)?))
}

#[get("/component/<id>")]
pub fn get_component(conn: MysqlConn, id: i32) -> Result<(Device, Subsystem, Component)> {
    Ok(Json(dao::get_component(&*conn, id)?))
}

#[get("/components?<query..>")]
pub fn query_component(conn: MysqlConn, query: Form<ComponentQuery>) -> Result<Vec<(Device, Subsystem, Component)>> {
    Ok(Json(dao::query_component(&*conn, query.0)?))
}




