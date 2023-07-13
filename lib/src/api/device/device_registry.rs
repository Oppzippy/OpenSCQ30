use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;
use macaddr::MacAddr6;

use super::{Device, DeviceDescriptor};

#[async_trait]
pub trait DeviceRegistry {
    type DeviceType: Device + Send + Sync + Debug;
    type DescriptorType: DeviceDescriptor + Send + Sync + Debug;

    async fn device_descriptors(&self) -> crate::Result<Vec<Self::DescriptorType>>;
    async fn device(&self, mac_address: MacAddr6) -> crate::Result<Option<Arc<Self::DeviceType>>>;
}
