use std::sync::Arc;

use openscq30_lib::{
    api::traits::SoundcoreDeviceRegistry,
    soundcore_bluetooth::traits::SoundcoreDeviceConnectionError,
};
use tokio::runtime::Runtime;

use super::soundcore_device::GtkSoundcoreDevice;

pub struct GtkSoundcoreDeviceRegistry<RegistryType: 'static>
where
    RegistryType: SoundcoreDeviceRegistry + Send + Sync,
{
    tokio_runtime: Runtime,
    soundcore_device_registry: Arc<RegistryType>,
}

impl<RegistryType: 'static> GtkSoundcoreDeviceRegistry<RegistryType>
where
    RegistryType: SoundcoreDeviceRegistry + Send + Sync,
{
    pub fn new(registry: RegistryType, tokio_runtime: Runtime) -> Self {
        Self {
            soundcore_device_registry: Arc::new(registry),
            tokio_runtime,
        }
    }

    pub async fn refresh_devices(&self) -> Result<(), SoundcoreDeviceConnectionError> {
        let device_registry = self.soundcore_device_registry.to_owned();
        async_runtime_bridge!(self.tokio_runtime, device_registry.refresh_devices().await)
    }

    pub async fn devices(&self) -> Vec<Arc<GtkSoundcoreDevice<RegistryType::DeviceType>>> {
        let device_registry = self.soundcore_device_registry.to_owned();
        let devices = async_runtime_bridge!(self.tokio_runtime, device_registry.devices().await);
        devices
            .into_iter()
            .map(|device| self.to_gtk_device(device))
            .collect()
    }

    pub async fn device_by_mac_address(
        &self,
        mac_address: &String,
    ) -> Option<Arc<GtkSoundcoreDevice<RegistryType::DeviceType>>> {
        let device_registry = self.soundcore_device_registry.to_owned();
        let mac_address = mac_address.to_owned();
        async_runtime_bridge!(
            self.tokio_runtime,
            device_registry.device_by_mac_address(&mac_address).await
        )
        .map(|device| self.to_gtk_device(device))
    }

    fn to_gtk_device(
        &self,
        device: Arc<RegistryType::DeviceType>,
    ) -> Arc<GtkSoundcoreDevice<RegistryType::DeviceType>> {
        Arc::new(GtkSoundcoreDevice::new(device, self.tokio_runtime.handle()))
    }
}
