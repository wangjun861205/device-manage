use super::chrono::NaiveDateTime;
use super::schema::*;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::*;
use diesel::*;
use serde::{Deserialize, Serialize};
use std::io::Write;
use rocket::request::*;
use std::default::Default;
use rocket::http::RawStr;



#[derive(Debug, Copy, Clone, AsExpression, FromSqlRow, Deserialize, Serialize)]
#[sql_type = "VarChar"]
pub enum DeviceStatus {
    Running,
    Stopped,
    Breakdown,
}

impl<DB> ToSql<VarChar, DB> for DeviceStatus
where
    DB: Backend,
    String: ToSql<VarChar, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        match *self {
            DeviceStatus::Running => "Running".to_owned().to_sql(out),
            DeviceStatus::Stopped => "Stopped".to_owned().to_sql(out),
            DeviceStatus::Breakdown => "Breakdown".to_owned().to_sql(out),
        }
    }
}

impl<DB> FromSql<VarChar, DB> for DeviceStatus
where
    DB: Backend,
    String: FromSql<VarChar, DB>,
{
    fn from_sql(val: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        match String::from_sql(val)?.as_ref() {
            "Running" => Ok(Self::Running),
            "Stopped" => Ok(Self::Stopped),
            "Breakdown" => Ok(Self::Breakdown),
            _ => Err("unknown device status".into()),
        }
    }
}

impl<'v> FromFormValue<'v> for DeviceStatus {
    type Error = String;

    fn from_form_value(s: &'v RawStr) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Running" => Ok(Self::Running),
            "Stopped" => Ok(Self::Stopped),
            "Breakdown" => Ok(Self::Breakdown),
            _ => Err("unknown device status".to_owned())
        }
    }
}



#[derive(Debug, Serialize, Deserialize)]
pub struct MyDatetime(pub NaiveDateTime);

impl<'v> FromFormValue<'v> for MyDatetime {
    type Error = chrono::ParseError;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        let t = NaiveDateTime::parse_from_str(form_value.as_str(), "%Y-%m-%d %H:%M:%S")?;
        Ok(MyDatetime(t))
    }
}

//  ===================================================================================

#[derive(Insertable, Debug, Deserialize)]
#[table_name = "device_info"]
pub struct DeviceInfoInsert {
    pub name: String,
    pub model: String,
    pub maintain_interval: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromForm)]
pub struct DeviceInfoQuery {
    pub name: Option<String>,
    pub model: Option<String>,
    pub maintain_interval_begin: Option<i32>,
    pub maintain_interval_end: Option<i32>,
    pub page: i64,
    pub size: i64,
}


#[derive(Queryable, Debug, Serialize, Deserialize, Identifiable)]
#[table_name = "device_info"]
pub struct DeviceInfo {
    pub id: i32,
    pub name: String,
    pub model: String,
    pub maintain_interval: i32,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "device_info"]
pub struct DeviceInfoUpdate {
    pub name: Option<String>,
    pub model: Option<String>,
    pub maintain_interval: Option<i32>,
}

#[derive(Insertable, Debug, Deserialize)]
#[table_name = "device"]
pub struct DeviceInsert {
    pub name: String,
    pub model: String,
    pub maintain_interval: i32,
    pub unicode: String,
    pub last_start_at: Option<NaiveDateTime>,
    pub last_stop_at: Option<NaiveDateTime>,
    pub total_duration: i32,
    pub status: DeviceStatus,
}


#[derive(Queryable, Debug, Deserialize, Serialize, Identifiable, Associations, Clone)]
#[table_name = "device"]
pub struct Device {
    pub id: i32,
    pub name: String,
    pub model: String,
    pub maintain_interval: i32,
    pub unicode: String,
    pub last_start_at: Option<NaiveDateTime>,
    pub last_stop_at: Option<NaiveDateTime>,
    pub total_duration: i32,
    pub status: DeviceStatus,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Default, Deserialize, Serialize, FromForm)]
pub struct DeviceQuery {
    pub name: Option<String>,
    pub model: Option<String>,
    pub maintain_interval_begin: Option<i32>,
    pub maintain_interval_end: Option<i32>,
    pub unicode: Option<String>,
    pub last_start_at_begin: Option<MyDatetime>,
    pub last_start_at_end: Option<MyDatetime>,
    pub last_stop_at_begin: Option<MyDatetime>,
    pub last_stop_at_end: Option<MyDatetime>,
    pub total_duration_begin: Option<i32>,
    pub total_duration_end: Option<i32>,
    pub status: Option<DeviceStatus>,
    pub page: Option<i64>,
    pub size: Option<i64>,
}



#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[table_name="device"]
pub struct DeviceUpdate {
    pub name: Option<String>,
    pub model: Option<String>,
    pub unicode: Option<String>,
    pub last_start_at: Option<NaiveDateTime>,
    pub last_stop_at: Option<NaiveDateTime>,
    pub total_duration: Option<i32>,
    pub status: Option<DeviceStatus>,
}

#[derive(Insertable, Debug, Deserialize, Serialize)]
#[table_name="subsystem_info"]
pub struct SubsystemInfoInsert {
    pub name: String,
    pub maintain_interval: i32,
}

#[derive(Queryable, Debug, Deserialize, Serialize, FromForm)]
pub struct SubsystemInfoQuery {
    pub device_info_name: Option<String>,
    pub device_info_model: Option<String>,
    pub device_info_maintain_interval_begin: Option<i32>,
    pub device_info_maintain_interval_end: Option<i32>,
    pub name: Option<String>,
    pub maintain_interval_begin: Option<i32>,
    pub maintain_interval_end: Option<i32>,
    pub component_info_name: Option<String>,
    pub component_info_model: Option<String>,
    pub component_info_maintain_interval_begin: Option<i32>,
    pub component_info_maintain_interval_end: Option<i32>,
    pub page: i64,
    pub size: i64,
}

#[derive(Debug, Deserialize, Serialize, AsChangeset, FromForm)]
#[table_name="subsystem_info"]
pub struct SubsystemInfoUpdate {
    pub name: Option<String>,
    pub maintain_interval: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Identifiable, Associations, Queryable)]
#[table_name="subsystem_info"]
pub struct SubsystemInfo {
    pub id: i32, 
    pub name: String,
    pub maintain_interval: i32,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}


#[derive(Insertable, Debug, Deserialize, Serialize)]
#[table_name = "subsystem"]
pub struct SubsystemInsert {
    pub device_id: i32,
    pub name: String,
    pub maintain_interval: i32,
}

#[derive(Debug, Deserialize, Serialize, AsChangeset)]
#[table_name="subsystem"]
pub struct SubsystemUpdate {
    pub name: Option<String>,
    pub maintain_interval: Option<i32>,
}


#[derive(Queryable, Debug, Deserialize, Serialize, Default, FromForm)]
pub struct SubsystemQuery {
    pub device_name: Option<String>,
    pub device_model: Option<String>,
    pub device_maintain_interval_begin: Option<i32>,
    pub device_maintain_interval_end: Option<i32>,
    pub device_unicode: Option<String>,
    pub device_last_start_at_begin: Option<MyDatetime>,
    pub device_last_start_at_end: Option<MyDatetime>,
    pub device_last_stop_at_begin: Option<MyDatetime>,
    pub device_last_stop_at_end: Option<MyDatetime>,
    pub device_total_duration_begin: Option<i32>,
    pub device_total_duration_end: Option<i32>,
    pub device_status: Option<DeviceStatus>,
    pub subsystem_name: Option<String>,
    pub maintain_interval_begin: Option<i32>,
    pub maintain_interval_end: Option<i32>,
    pub page: Option<i64>,
    pub size: Option<i64>,
}




#[derive(Queryable, Debug, Deserialize, Serialize, Associations, Identifiable, Clone)]
#[table_name="subsystem"]
#[belongs_to(Device)]
pub struct Subsystem {
    pub id: i32,
    pub device_id: i32,
    pub name: String,
    pub maintain_interval: i32,
    pub create_at: NaiveDateTime,
    pub udpate_at: NaiveDateTime,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct ComponentInfo {
    pub id: i32,
    pub name: String,
    pub model: String,
    pub maintain_interval: i32,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}

#[derive(Insertable, Debug, Deserialize, Serialize)]
#[table_name="component_info"]
pub struct ComponentInfoInsert {
    pub name: String,
    pub model: String,
    pub maintain_interval: i32,
}

#[derive(Debug, Deserialize, Serialize, AsChangeset)]
#[table_name="component_info"]
pub struct ComponentInfoUpdate {
    pub name: Option<String>,
    pub model: Option<String>,
    pub maintain_interval: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromForm)]
pub struct ComponentInfoQuery {
    pub device_info_name: Option<String>,
    pub device_info_model: Option<String>,
    pub device_info_maintain_interval_begin: Option<i32>,
    pub device_info_maintain_interval_end: Option<i32>,
    pub subsystem_info_name: Option<String>,
    pub subsystem_info_maintain_interval_begin: Option<i32>,
    pub subsystem_info_maintain_interval_end: Option<i32>,
    pub name: Option<String>,
    pub model: Option<String>,
    pub maintain_interval_begin: Option<i32>,
    pub maintain_interval_end: Option<i32>,
    pub page: Option<i64>,
    pub size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Associations, Identifiable)]
#[table_name="component"]
#[belongs_to(Subsystem)]
pub struct Component {
    pub id: i32,
    pub subsystem_id: i32,
    pub name: String,
    pub model: String,
    pub maintain_interval: i32,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "component"]
pub struct ComponentInsert {
    pub subsystem_id: i32,
    pub name: String,
    pub model: String,
    pub maintain_interval: i32,
}

#[derive(Debug, Serialize, Deserialize, FromForm)]
pub struct ComponentQuery {
    pub device_name: Option<String>,
    pub device_model: Option<String>,
    pub device_maintain_interval_begin: Option<i32>,
    pub device_maintain_interval_end: Option<i32>,
    pub subsystem_name: Option<String>,
    pub subsystem_maintain_interval_begin: Option<i32>,
    pub subsystem_maintain_interval_end: Option<i32>,
    pub name: Option<String>,
    pub model: Option<String>,
    pub maintain_interval_begin: Option<i32>,
    pub maintain_interval_end: Option<i32>,
    pub page: Option<i64>,
    pub size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[table_name="component"]
pub struct ComponentUpdate {
    pub name: Option<String>,
    pub model: Option<String>,
    pub maintain_interval: Option<i32>,
}



#[derive(Debug, Serialize, Deserialize, Associations, Identifiable, Queryable)]
#[table_name="deviceinfo_subsysteminfo"]
#[belongs_to(DeviceInfo)]
#[belongs_to(SubsystemInfo)]
pub struct DeviceinfoSubsysteminfo {
    pub id: i32,
    pub device_info_id: i32,
    pub subsystem_info_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Associations, Identifiable, Queryable)]
#[table_name="subsysteminfo_componentinfo"]
#[belongs_to(SubsystemInfo)]
#[belongs_to(ComponentInfo)]
pub struct SubsysteminfoComponentinfo {
    pub id: i32,
    pub subsystem_info_id: i32,
    pub component_info_id: i32,
    pub quantity: i32,
}