use super::dao;
use super::model::*;
use super::result::Result;
use super::service;
use super::storer::mysqlstorer;
use super::MysqlConn;
use rocket::request::Form;
use rocket_contrib::json::Json;

// ===============================================device_info=================================================

// #[post("/device_info", format = "application/json", data = "<info>")]
// pub fn add_device_info(conn: MysqlConn, info: Json<DeviceInfoInsert>) -> Result<usize> {
//     Ok(Json(dao::insert_device_info(&*conn, info.0)?))
// }

// #[post("/device_infos", format="application/json", data="<infos>", rank = 1)]
// pub fn bulk_add_device_info(conn: MysqlConn, infos: Json<Vec<DeviceInfoInsert>>) -> Result<usize> {
//     Ok(Json(dao::bulk_insert_device_info(&*conn, &infos.0)?))
// }

// #[delete("/device_info/<id>")]
// pub fn delete_device_info(conn: MysqlConn, id: i32) -> Result<usize> {
//     Ok(Json(dao::delete_device_info(&*conn, id)?))
// }

// #[put("/device_info/<id>", format="application/json", data="<req>")]
// pub fn update_device_info(conn: MysqlConn, id: i32, req: Json<DeviceInfoUpdate>) -> Result<usize> {
//     Ok(Json(dao::update_device_info(&*conn, id, req.0)?))
// }

// #[get("/device_info/<id>")]
// pub fn get_device_info(conn: MysqlConn, id: i32) -> Result<DeviceInfo> {
//     Ok(Json(dao::get_device_info(&*conn, id)?))
// }

// #[get("/device_infos?<query..>")]
// pub fn query_device_info(conn: MysqlConn, query: Form<DeviceInfoQuery>) -> Result<(Vec<DeviceInfo>, i64)> {
//     Ok(Json(dao::query_device_infos(&*conn, query.0)?))
// }

// #[get("/subsystem_info/<subinfoid>/device_infos?<query..>")]
// pub fn query_device_info_by_subsystem_info(conn: MysqlConn, subinfoid: i32, query: Form<DeviceInfoQuery>) -> Result<(Vec<DeviceInfo>, i64)> {
//     Ok(Json(dao::query_device_infos_by_subsystem_info(&*conn, subinfoid, query.0)?))
// }

// //==============================================device===========================================================

// #[post("/device", format = "application/json", data = "<dev>")]
// pub fn add_device(conn: MysqlConn, dev: Json<DeviceInsert>) -> Result<usize> {
//     Ok(Json(dao::insert_device(&*conn, dev.0)?))
// }

// #[post("/devices", format="application/json", data="<devs>")]
// pub fn bulk_add_device(conn: MysqlConn, devs: Json<Vec<DeviceInsert>>) -> Result<usize> {
//     Ok(Json(dao::bulk_insert_device(&*conn, &devs.0)?))
// }

// #[delete("/device/<id>")]
// pub fn delete_device(conn: MysqlConn, id: i32) -> Result<usize> {
//     Ok(Json(dao::delete_device(&*conn, id)?))
// }

// #[put("/device/<id>", format="application/json", data="<req>")]
// pub fn update_device(conn: MysqlConn, id: i32, req: Json<DeviceUpdate>) -> Result<usize> {
//     Ok(Json(dao::update_device(&*conn, id, req.0)?))
// }

// #[get("/device/<id>")]
// pub fn get_device(conn: MysqlConn, id: i32) -> Result<(Device, Vec<(Subsystem, Vec<Component>)>)> {
//     Ok(Json(dao::get_device(&*conn, id)?))
// }

// #[get("/devices?<query..>")]
// pub fn query_device(conn: MysqlConn, query: Form<DeviceQuery>) -> Result<Vec<(Device, Vec<(Subsystem, Vec<Component>)>)>> {
//     Ok(Json(dao::query_device(&*conn, query.0)?))
// }

// //==================================================subsystem_info========================================================

// #[post("/subsystem_info", format="application/json", data="<info>")]
// pub fn add_subsystem_info(conn: MysqlConn, info: Json<SubsystemInfoInsert>) -> Result<usize> {
//     Ok(Json(dao::insert_subsystem_info(&*conn, info.0)?))
// }

// #[post("/subsystem_infos", format="application/json", data="<infos>")]
// pub fn bulk_add_subsystem_info(conn: MysqlConn, infos: Json<Vec<SubsystemInfoInsert>>) -> Result<usize> {
//     Ok(Json(dao::bulk_insert_subsystem_info(&*conn, &infos.0)?))
// }

// #[delete("/subsystem_info/<id>")]
// pub fn delete_subsystem_info(conn: MysqlConn, id: i32) -> Result<usize> {
//     Ok(Json(dao::delete_subsystem_info(&*conn, id)?))
// }

// #[put("/subsystem_info/<id>", format="application/json", data="<upd>")]
// pub fn update_subsystem_info(conn: MysqlConn, id: i32, upd: Json<SubsystemInfoUpdate>) -> Result<usize> {
//     Ok(Json(dao::update_subsystem_info(&*conn, id, upd.0)?))
// }

// #[get("/subsysetem_info/<id>")]
// pub fn get_subsystem_info(conn: MysqlConn, id: i32) -> Result<(SubsystemInfo, Vec<DeviceInfo>, Vec<ComponentInfo>)> {
//     Ok(Json(dao::get_subsystem_info(&*conn, id)?))
// }

// #[get("/subsystem_infos?<query..>")]
// pub fn query_subsystem_info(conn: MysqlConn, query: Form<SubsystemInfoQuery>) -> Result<(Vec<SubsystemInfo>, i64)> {
//     Ok(Json(dao::query_subsystem_infos(&*conn, query.0)?))
// }

// #[get("/device_info/<devinfoid>/subsystem_infos?<query..>")]
// pub fn query_subsystem_info_by_device_info(conn: MysqlConn, devinfoid: i32, query: Form<SubsystemInfoQuery>) -> Result<(Vec<SubsystemInfo>, i64)> {
//     Ok(Json(dao::query_subsystem_infos_by_device_info(&*conn, devinfoid, query.0)?))
// }

// #[get("/component_info/<cominfoid>/subsystem_infos?<query..>")]
// pub fn query_subsystem_info_by_component_info(conn: MysqlConn, cominfoid: i32, query: Form<SubsystemInfoQuery>) -> Result<(Vec<SubsystemInfo>, i64)> {
//     Ok(Json(dao::query_subsystem_infos_by_component_info(&*conn, cominfoid, query.0)?))
// }

// // =============================================================subsystem=============================================================

// #[post("/subsystem", format="application/json", data="<sys>")]
// pub fn add_subsystem(conn: MysqlConn, sys: Json<SubsystemInsert>) -> Result<usize> {
//     Ok(Json(dao::insert_subsystem(&*conn, sys.0)?))
// }

// #[post("/subsystems", format="application/json", data="<ss>")]
// pub fn bulk_add_subsystem(conn: MysqlConn, ss: Json<Vec<SubsystemInsert>>) -> Result<usize> {
//     Ok(Json(dao::bulk_insert_subsystem(&*conn, &ss.0)?))
// }

// #[delete("/subsystem/<id>")]
// pub fn delete_subsystem(conn: MysqlConn, id: i32) -> Result<usize> {
//     Ok(Json(dao::delete_subsystem(&*conn, id)?))
// }

// #[put("/subsystem/<id>", format="application/json", data="<upd>")]
// pub fn update_subsystem(conn: MysqlConn, id: i32, upd: Json<SubsystemUpdate>) -> Result<usize> {
//     Ok(Json(dao::udpate_subsystem(&*conn, id, upd.0)?))
// }

// #[get("/subsystem/<id>")]
// pub fn get_subsystem(conn: MysqlConn, id: i32) -> Result<(Device, Subsystem, Vec<Component>)> {
//     Ok(Json(dao::get_subsystem(&*conn, id)?))
// }

// #[get("/subsystems?<query..>")]
// pub fn query_subsystem(conn: MysqlConn, query: Form<SubsystemQuery>) -> Result<Vec<(Device, Subsystem, Vec<Component>)>> {
//     Ok(Json(dao::query_subsystem(&*conn, query.0)?))
// }

// // ===================================================component_info=======================================================

// #[post("/component_info", format="application/json", data="<info>")]
// pub fn add_component_info(conn: MysqlConn, info: Json<ComponentInfoInsert>) -> Result<usize> {
//     Ok(Json(dao::insert_component_info(&*conn, info.0)?))
// }

// #[post("/component_infos", format="application/json", data="<infos>")]
// pub fn bulk_add_component_info(conn: MysqlConn, infos: Json<Vec<ComponentInfoInsert>>) -> Result<usize> {
//     Ok(Json(dao::bulk_insert_component_info(&*conn, &infos.0)?))
// }

// #[delete("/component_info/<id>")]
// pub fn delete_component_info(conn: MysqlConn, id: i32) -> Result<usize> {
//     Ok(Json(dao::delete_component_info(&*conn, id)?))
// }

// #[put("/component_info/<id>", format="application/json", data="<upd>")]
// pub fn update_component_info(conn: MysqlConn, id: i32, upd: Json<ComponentInfoUpdate>) -> Result<usize> {
//     Ok(Json(dao::update_component_info(&*conn, id, upd.0)?))
// }

// #[get("/component_info/<id>")]
// pub fn get_component_info(conn: MysqlConn, id: i32) -> Result<(DeviceInfo, SubsystemInfo, ComponentInfo)> {
//     Ok(Json(dao::get_component_info(&*conn, id)?))
// }

// #[get("/component_info?<query..>")]
// pub fn query_component_info(conn: MysqlConn, query: Form<ComponentInfoQuery>) -> Result<(Vec<ComponentInfo>, i64)> {
//     Ok(Json(dao::query_component_infos(&*conn, query.0)?))
// }

// #[get("/subsystem_info/<subinfoid>/component_infos?<query..>")]
// pub fn query_component_info_by_subsystem_info(conn: MysqlConn, subinfoid: i32, query: Form<ComponentInfoQuery>) -> Result<(Vec<ComponentInfo>, i64)> {
//     Ok(Json(dao::query_component_infos_by_subsystem_info(&*conn, subinfoid, query.0)?))
// }

// #[post("/component", format="application/json", data="<com>")]
// pub fn add_component(conn: MysqlConn, com: Json<ComponentInsert>) -> Result<usize> {
//     Ok(Json(dao::insert_component(&*conn, com.0)?))
// }

// #[post("/components", format="application/json", data="<coms>")]
// pub fn bulk_add_component(conn: MysqlConn, coms: Json<Vec<ComponentInsert>>) -> Result<usize> {
//     Ok(Json(dao::bulk_insert_component(&*conn, &coms.0)?))
// }

// #[delete("/component/<id>")]
// pub fn delete_component(conn: MysqlConn, id: i32) -> Result<usize> {
//     Ok(Json(dao::delete_component(&*conn, id)?))
// }

// #[put("/component/<id>", format="application/json", data="<upd>")]
// pub fn update_component(conn: MysqlConn, id: i32, upd: Json<ComponentUpdate>) -> Result<usize> {
//     Ok(Json(dao::update_component(&*conn, id, upd.0)?))
// }

// #[get("/component/<id>")]
// pub fn get_component(conn: MysqlConn, id: i32) -> Result<(Device, Subsystem, Component)> {
//     Ok(Json(dao::get_component(&*conn, id)?))
// }

// #[get("/components?<query..>")]
// pub fn query_component(conn: MysqlConn, query: Form<ComponentQuery>) -> Result<Vec<(Device, Subsystem, Component)>> {
//     Ok(Json(dao::query_component(&*conn, query.0)?))
// }
