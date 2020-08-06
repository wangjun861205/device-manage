use super::dao::*;
use super::model::*;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub trait Server {
    fn add_component_info(&self, name: String, model: String, interval: i32) -> Result<usize>;
    fn delete_component_info(&self, compinfo_id: i32) -> Result<usize>;
    fn add_subsystem_info(&self, name: String, interval: i32) -> Result<usize>;
    fn delete_subsystem_info(&self, subinfo_id: i32) -> Result<usize>;
    fn add_device_info(&self, name: String, model: String, interval: i32) -> Result<usize>;
    fn delete_device_info(&self, devinfo_id: i32) -> Result<usize>;
    fn attach_subsystem_info(&self, devinfo_id: i32, subinfo_id: i32) -> Result<usize>;
    fn remove_subsystem_info(&self, devinfo_id: i32, subinfo_id: i32) -> Result<usize>;
    fn attach_component_info(&self, devinfo_id: i32, subinfo_id: i32, cominfo_id: i32, quantity: i32) -> Result<usize>;
    fn remove_component_info(&self, devinfo_id: i32, subinfo_id: i32, cominfo_id: i32) -> Result<usize>;
    fn create_device(&self, devinfo_id: i32) -> Result<usize>;
    fn delete_device(&self, dev_id: i32) -> Result<usize>;
}

pub struct Service {
    pub devinfo: Box<dyn DeviceInfoStorer>,
    pub subinfo: Box<dyn SubsystemInfoStorer>,
    pub cominfo: Box<dyn ComponentInfoStorer>,
    pub dev: Box<dyn DeviceInfoStorer>,
    pub sub: Box<dyn SubsystemStorer>,
    pub com: Box<dyn ComponentStorer>,
    pub rel: Box<dyn RelationStorer>,
}

impl Server for Service {
    fn add_device_info(&self, name: String, model: String, interval: i32) -> Result<usize> {
        Ok(self.devinfo.insert(DeviceInfoInsert {
            name: name,
            model: model,
            maintain_interval: interval,
        })?)
    }

    fn delete_device_info(&self, devinfo_id: i32) -> Result<usize> {
        Ok(self.devinfo.delete(devinfo_id)?)
    }

    fn add_subsystem_info(&self, name: String, interval: i32) -> Result<usize> {
        Ok(self.subinfo.insert(SubsystemInfoInsert {
            name: name,
            maintain_interval: interval,
        })?)
    }

    fn delete_subsystem_info(&self, subinfo_id: i32) -> Result<usize> {
        Ok(self.subinfo.delete(subinfo_id)?)
    }

    fn add_component_info(&self, name: String, model: String, interval: i32) -> Result<usize> {
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
}
