use std::{fmt::Debug, rc::Rc};

use async_trait::async_trait;
use macaddr::MacAddr6;

use super::{Device, DeviceDescriptor};

#[async_trait(?Send)]
pub trait DeviceRegistry {
    type DeviceType: Device + Debug;
    type DescriptorType: DeviceDescriptor + Send + Sync + Debug;

    async fn device_descriptors(&self) -> crate::Result<Vec<Self::DescriptorType>>;
    async fn device(&self, mac_address: MacAddr6) -> crate::Result<Option<Rc<Self::DeviceType>>>;
}
