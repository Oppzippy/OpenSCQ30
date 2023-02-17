use std::{collections::HashSet, fmt::Debug, sync::Arc};

use async_trait::async_trait;

use super::{
    soundcore_device_connection::SoundcoreDeviceConnection,
    soundcore_device_connection_error::SoundcoreDeviceConnectionError,
    SoundcoreDeviceConnectionDescriptor,
};

#[async_trait]
pub trait SoundcoreDeviceConnectionRegistry {
    type DeviceConnectionType: SoundcoreDeviceConnection + Send + Sync;
    type DescriptorType: SoundcoreDeviceConnectionDescriptor + Debug + Send + Sync;

    async fn connection_descriptors(
        &self,
    ) -> Result<HashSet<Self::DescriptorType>, SoundcoreDeviceConnectionError>;

    async fn connection(
        &self,
        mac_address: &str,
    ) -> Result<Option<Arc<Self::DeviceConnectionType>>, SoundcoreDeviceConnectionError>;
}
