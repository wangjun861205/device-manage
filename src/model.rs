use super::chrono::NaiveDateTime;
use super::schema::*;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::*;
use diesel::*;
use serde::{Deserialize, Serialize};
use std::io::Write;
#[macro_use]
use rocket::request::*;
use std::default::Default;
use std::convert::From;
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

#[derive(Insertable, Debug, Deserialize)]
#[table_name = "device_info"]
pub struct DeviceInfoInsert {
    pub name: String,
    pub model: String,
    pub maintain_interval: Option<i32>,
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
    pub device_info_id: i32,
    pub unicode: String,
    pub last_start_at: Option<NaiveDateTime>,
    pub last_stop_at: Option<NaiveDateTime>,
    pub total_duration: i32,
    pub status: DeviceStatus,
}

#[derive(Queryable, Debug, Deserialize, Serialize, Identifiable, Associations)]
#[table_name = "device"]
pub struct Device {
    pub id: i32,
    pub device_info_id: i32,
    pub unicode: String,
    pub last_start_at: Option<NaiveDateTime>,
    pub last_stop_at: Option<NaiveDateTime>,
    pub total_duration: i32,
    pub status: DeviceStatus,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Default)]
pub struct DeviceQuery {
    pub name: Option<String>,
    pub model: Option<String>,
    pub maintain_interval: Option<i32>,
    pub unicode: Option<String>,
    pub last_start_at: Option<NaiveDateTime>,
    pub last_stop_at: Option<NaiveDateTime>,
    pub total_duration: Option<i32>,
    pub status: Option<DeviceStatus>,
    pub page: Option<i64>,
    pub size: Option<i64>,
}

impl<'q> FromQuery<'q> for DeviceQuery {
    type Error = ();
    fn from_query(query: Query<'q>) -> Result<Self, Self::Error> {
        let mut dq = DeviceQuery{
            ..Default::default()
        };
        for q in query.into_iter() {
            match q.key.as_str() {
                "name" => dq.name = Some(q.value.url_decode().unwrap().to_string()),
                "model" => dq.model = Some(q.value.url_decode().unwrap().to_string()),
                "maintain_interval" => {
                    match q.value.to_string().parse::<i32>() {
                        Ok(i) => dq.maintain_interval = Some(i),
                        Err(_) => { return Err(()) }
                    }
                },
                "unicode" => dq.unicode = Some(q.value.to_string()),
                "last_start_at" => {
                    match NaiveDateTime::parse_from_str(q.value.url_decode().unwrap().as_str(), "%Y-%m-%d %H:%M:%S") {
                        Ok(t) => dq.last_start_at = Some(t),
                        Err(_) => { return Err(()) },
                    }
                },
                "last_stop_at" => {
                    match NaiveDateTime::parse_from_str(q.value.url_decode().unwrap().as_str(), "%Y-%m-%d %H:%M:%S") {
                        Ok(t) => dq.last_stop_at = Some(t),
                        Err(_) => { return Err(()) },
                    }
                },
                "total_duration" => {
                    match q.value.to_string().parse::<i32>() {
                        Ok(i) => dq.total_duration = Some(i),
                        Err(_) => { return Err(()) }
                    }
                },
                "status" => {
                    match q.value.as_str() {
                        "Running" => dq.status = Some(DeviceStatus::Running),
                        "Stopped" => dq.status = Some(DeviceStatus::Stopped),
                        "Breakdown" => dq.status = Some(DeviceStatus::Breakdown),
                        _ => { return Err(()) },
                    }
                },
                "page" => {
                    match q.value.to_string().parse::<i64>() {
                        Ok(i) => dq.page = Some(i),
                        Err(_) => { return Err(()) }
                    }
                },
                "size" => {
                    match q.value.to_string().parse::<i64>() {
                        Ok(i) => dq.size = Some(i),
                        Err(_) => { return Err(()) }
                    }
                },
                _ => {},
            }
        }
        Ok(dq)
    }
}


#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[table_name="device"]
pub struct DeviceUpdate {
    pub device_info_id: Option<i32>,
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
    pub name: Option<String>,
    pub maintain_interval: Option<i32>,
    pub page: Option<i64>,
    pub size: Option<i64>,
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
    pub subsystem_info_id: i32,
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




#[derive(Queryable, Debug, Deserialize, Serialize, Default, FromForm)]
pub struct SubsystemQuery {
    pub device_name: Option<String>,
    pub device_model: Option<String>,
    pub device_maintain_interval: Option<i32>,
    pub device_unicode: Option<String>,
    pub device_last_start_at: Option<MyDatetime>,
    pub device_last_stop_at: Option<MyDatetime>,
    pub device_total_duration: Option<i32>,
    pub device_status: Option<DeviceStatus>,
    pub subsystem_name: Option<String>,
    pub subsystem_maintain_interval: Option<i32>,
    pub page: Option<i64>,
    pub size: Option<i64>,
}




#[derive(Queryable, Debug, Deserialize, Serialize, Associations, Identifiable)]
#[table_name="subsystem"]
pub struct Subsystem {
    pub id: i32,
    pub device_id: i32,
    pub subsystem_info_id: i32,
    pub create_at: NaiveDateTime,
    pub udpate_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "component_info"]
pub struct ComponentInfo {
    pub name: String,
    pub model: String,
}

#[derive(Queryable, Debug)]
pub struct ComponentInfoQuery<'a> {
    pub id: i32,
    pub name: &'a str,
}

#[derive(Insertable, Debug)]
#[table_name = "component"]
pub struct ComponentInsert {
    pub component_info_id: i32,
    pub subsystem_id: i32,
}

#[derive(Queryable, Debug)]
pub struct ComponentQuery {
    pub id: i32,
    pub subsystem_id: i32,
    pub component_info_id: String,
}
