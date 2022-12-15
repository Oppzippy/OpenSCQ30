use std::sync::Arc;

use async_trait::async_trait;

use super::{
    soundcore_device_connection::SoundcoreDeviceConnection,
    soundcore_device_connection_error::SoundcoreDeviceConnectionError,
};

#[async_trait]
pub trait SoundcoreDeviceConnectionRegistry {
    type DeviceConnectionType: SoundcoreDeviceConnection + Send + Sync;

    async fn refresh_connections(&self) -> Result<(), SoundcoreDeviceConnectionError>;
    async fn connections(&self) -> Vec<Arc<Self::DeviceConnectionType>>;
}
