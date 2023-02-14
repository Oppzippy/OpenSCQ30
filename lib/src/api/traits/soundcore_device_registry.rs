use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;

use crate::soundcore_bluetooth::traits::SoundcoreDeviceConnectionError;

use super::SoundcoreDevice;

#[async_trait]
pub trait SoundcoreDeviceRegistry {
    type DeviceType: SoundcoreDevice + Send + Sync + Debug;

    async fn refresh_devices(&self) -> Result<(), SoundcoreDeviceConnectionError>;

    async fn devices(&self) -> Vec<Arc<Self::DeviceType>>;

    async fn device_by_mac_address(&self, mac_address: &String) -> Option<Arc<Self::DeviceType>>;
}
