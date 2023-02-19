use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;

use super::{SoundcoreDevice, SoundcoreDeviceDescriptor};

#[async_trait]
pub trait SoundcoreDeviceRegistry {
    type DeviceType: SoundcoreDevice + Send + Sync + Debug;
    type DescriptorType: SoundcoreDeviceDescriptor + Send + Sync + Debug;

    async fn device_descriptors(&self) -> crate::Result<Vec<Self::DescriptorType>>;
    async fn device(&self, mac_address: &str) -> crate::Result<Option<Arc<Self::DeviceType>>>;
}
