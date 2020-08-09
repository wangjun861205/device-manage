use super::dao::*;
use super::model::*;
use rocket::request::{FromRequest, Outcome, Request};
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub trait Server {
    fn add_component_info(&self, name: String, model: String, interval: i32) -> Result<i32>;
    fn delete_component_info(&self, compinfo_id: i32) -> Result<usize>;
    fn add_subsystem_info(&self, name: String, interval: i32) -> Result<i32>;
    fn delete_subsystem_info(&self, subinfo_id: i32) -> Result<usize>;
    fn add_device_info(&self, name: String, model: String, interval: i32) -> Result<i32>;
    fn delete_device_info(&self, devinfo_id: i32) -> Result<usize>;
    fn attach_subsystem_info(&self, devinfo_id: i32, subinfo_id: i32) -> Result<usize>;
    fn remove_subsystem_info(&self, devinfo_id: i32, subinfo_id: i32) -> Result<usize>;
    fn attach_component_info(&self, devinfo_id: i32, subinfo_id: i32, cominfo_id: i32, quantity: i32) -> Result<usize>;
    fn remove_component_info(&self, devinfo_id: i32, subinfo_id: i32, cominfo_id: i32) -> Result<usize>;
    fn create_device(&self, devinfo_id: i32, unicode: String) -> Result<()>;
    fn delete_device(&self, dev_id: i32) -> Result<usize>;
}

pub struct Service {
    pub devinfo: Box<dyn DeviceInfoStorer>,
    pub subinfo: Box<dyn SubsystemInfoStorer>,
    pub cominfo: Box<dyn ComponentInfoStorer>,
    pub dev: Box<dyn DeviceStorer>,
    pub sub: Box<dyn SubsystemStorer>,
    pub com: Box<dyn ComponentStorer>,
    pub rel: Box<dyn RelationStorer>,
}

impl Service {
    pub fn new(
        devinfo: Box<dyn DeviceInfoStorer>,
        subinfo: Box<dyn SubsystemInfoStorer>,
        cominfo: Box<dyn ComponentInfoStorer>,
        dev: Box<dyn DeviceStorer>,
        sub: Box<dyn SubsystemStorer>,
        com: Box<dyn ComponentStorer>,
        rel: Box<dyn RelationStorer>,
    ) -> Self {
        Service {
            devinfo,
            subinfo,
            cominfo,
            dev,
            sub,
            com,
            rel,
        }
    }
}

use super::storer::mysqlstorer;
use std::rc::Rc;
use diesel::r2d2::{ConnectionManager, Pool};
use rocket::State;
use diesel::MysqlConnection;

impl<'a, 'r> FromRequest<'a, 'r> for Service {
    type Error = ();
    fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let pool = req.guard::<State<Pool<ConnectionManager<MysqlConnection>>>>()?;
        let conn = Rc::new(pool.get().unwrap());
        let svc = Service {
            devinfo: Box::new(mysqlstorer::DeviceInfoRepository::new(conn.clone())),
            subinfo: Box::new(mysqlstorer::SubsystemInfoRepository::new(conn.clone())),
            cominfo: Box::new(mysqlstorer::ComponentInfoRepository::new(conn.clone())),
            dev: Box::new(mysqlstorer::DeviceRepository::new(conn.clone())),
            sub: Box::new(mysqlstorer::SubsystemRepository::new(conn.clone())),
            com: Box::new(mysqlstorer::ComponentRepository::new(conn.clone())),
            rel: Box::new(mysqlstorer::RelationRepository::new(conn.clone())),
        };
        Outcome::Success(svc)
    }
}

impl Server for Service {
    fn add_device_info(&self, name: String, model: String, interval: i32) -> Result<i32> {
        Ok(self.devinfo.insert(DeviceInfoInsert {
            name: name,
            model: model,
            maintain_interval: interval,
        })?)
    }

    fn delete_device_info(&self, devinfo_id: i32) -> Result<usize> {
        Ok(self.devinfo.delete(devinfo_id)?)
    }

    fn add_subsystem_info(&self, name: String, interval: i32) -> Result<i32> {
        Ok(self.subinfo.insert(SubsystemInfoInsert {
            name: name,
            maintain_interval: interval,
        })?)
    }

    fn delete_subsystem_info(&self, subinfo_id: i32) -> Result<usize> {
        Ok(self.subinfo.delete(subinfo_id)?)
    }

    fn add_component_info(&self, name: String, model: String, interval: i32) -> Result<i32> {
        Ok(self.cominfo.insert(ComponentInfoInsert {
            name: name,
            model: model,
            maintain_interval: interval,
        })?)
    }

    fn delete_component_info(&self, cominfo_id: i32) -> Result<usize> {
        Ok(self.cominfo.delete(cominfo_id)?)
    }

    fn attach_subsystem_info(&self, devinfo_id: i32, subinfo_id: i32) -> Result<usize> {
        self.devinfo.get(devinfo_id)?;
        self.subinfo.get(subinfo_id)?;
        Ok(self.rel.insert_deviceinfo_subsysteminfo(DevinfoSubinfoInsert {
            device_info_id: devinfo_id,
            subsystem_info_id: subinfo_id,
        })?)
    }

    fn remove_subsystem_info(&self, devinfo_id: i32, subinfo_id: i32) -> Result<usize> {
        self.rel.delete_deviceinfo_subsysteminfo(devinfo_id, subinfo_id)?;
        Ok(self.rel.bulk_delete_subsysteminfo_componentinfo(devinfo_id, subinfo_id)?)
    }

    fn attach_component_info(&self, devinfo_id: i32, subinfo_id: i32, cominfo_id: i32, quantity: i32) -> Result<usize> {
        self.subinfo.get(subinfo_id)?;
        self.cominfo.get(cominfo_id)?;
        Ok(self.rel.insert_subsysteminfo_componentinfo(SubinfoCominfoInsert {
            device_info_id: devinfo_id,
            subsystem_info_id: subinfo_id,
            component_info_id: cominfo_id,
            quantity: quantity,
        })?)
    }

    fn remove_component_info(&self, devinfo_id: i32, subinfo_id: i32, cominfo_id: i32) -> Result<usize> {
        Ok(self.rel.delete_subsysteminfo_componentinfo(devinfo_id, subinfo_id, cominfo_id)?)
    }

    fn create_device(&self, devinfo_id: i32, unicode: String) -> Result<()> {
        let devinfo = self.devinfo.detail(devinfo_id)?;
        let devins = DeviceInsert {
            name: devinfo.0.name,
            model: devinfo.0.model,
            maintain_interval: devinfo.0.maintain_interval,
            unicode: unicode,
            last_start_at: None,
            last_stop_at: None,
            total_duration: 0,
            status: DeviceStatus::Stopped,
        };
        let devid = self.dev.insert(devins)?;
        for subinfo in devinfo.1 {
            let subins = SubsystemInsert {
                device_id: devid,
                name: subinfo.0.name,
                maintain_interval: subinfo.0.maintain_interval,
            };
            let subid = self.sub.insert(subins)?;
            for cominfo in subinfo.1 {
                let comins = ComponentInsert {
                    subsystem_id: subid,
                    name: cominfo.name,
                    model: cominfo.model,
                    maintain_interval: cominfo.maintain_interval,
                };
                self.com.insert(comins)?;
            }
        }
        Ok(())
    }

    fn delete_device(&self, id: i32) -> Result<usize> {
        Ok(self.dev.delete(id)?)
    }
}
