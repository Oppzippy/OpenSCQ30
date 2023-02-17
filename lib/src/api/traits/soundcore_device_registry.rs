use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;

use crate::soundcore_bluetooth::traits::SoundcoreDeviceConnectionError;

use super::{SoundcoreDevice, SoundcoreDeviceDescriptor};

#[async_trait]
pub trait SoundcoreDeviceRegistry {
    type DeviceType: SoundcoreDevice + Send + Sync + Debug;
    type DescriptorType: SoundcoreDeviceDescriptor + Send + Sync + Debug;

    async fn device_descriptors(
        &self,
    ) -> Result<Vec<Self::DescriptorType>, SoundcoreDeviceConnectionError>;

    async fn device(
        &self,
        mac_address: &str,
    ) -> Result<Option<Arc<Self::DeviceType>>, SoundcoreDeviceConnectionError>;
}
