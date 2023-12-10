use std::{fmt::Debug, rc::Rc};

use macaddr::MacAddr6;

use super::{Device, DeviceDescriptor};

pub trait DeviceRegistry {
    type DeviceType: Device + Debug;
    type DescriptorType: DeviceDescriptor + Send + Sync + Debug;

    async fn device_descriptors(&self) -> crate::Result<Vec<Self::DescriptorType>>;
    async fn device(&self, mac_address: MacAddr6) -> crate::Result<Option<Rc<Self::DeviceType>>>;
}
