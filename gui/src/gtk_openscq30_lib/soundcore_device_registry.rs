use std::sync::Arc;

use openscq30_lib::{
    api::{SoundcoreDevice, SoundcoreDeviceRegistry},
    soundcore_bluetooth::traits::SoundcoreDeviceConnectionError,
};
use tokio::runtime::Runtime;

use super::soundcore_device::GtkSoundcoreDevice;

pub struct GtkSoundcoreDeviceRegistry {
    tokio_runtime: Runtime,
    soundcore_device_registry: Arc<SoundcoreDeviceRegistry>,
}

impl GtkSoundcoreDeviceRegistry {
    pub fn new(registry: SoundcoreDeviceRegistry, tokio_runtime: Runtime) -> Self {
        Self {
            soundcore_device_registry: Arc::new(registry),
            tokio_runtime,
        }
    }

    pub async fn refresh_devices(&self) -> Result<(), SoundcoreDeviceConnectionError> {
        let device_registry = self.soundcore_device_registry.to_owned();
        async_runtime_bridge!(self.tokio_runtime, device_registry.refresh_devices().await)
    }

    pub async fn get_devices(&self) -> Vec<Arc<GtkSoundcoreDevice>> {
        let device_registry = self.soundcore_device_registry.to_owned();
        let devices =
            async_runtime_bridge!(self.tokio_runtime, device_registry.get_devices().await);
        devices
            .into_iter()
            .map(|device| self.to_gtk_device(device))
            .collect()
    }

    pub async fn get_device_by_mac_address(
        &self,
        mac_address: &String,
    ) -> Option<Arc<GtkSoundcoreDevice>> {
        let device_registry = self.soundcore_device_registry.to_owned();
        let mac_address = mac_address.to_owned();
        async_runtime_bridge!(
            self.tokio_runtime,
            device_registry
                .get_device_by_mac_address(&mac_address)
                .await
        )
        .map(|device| self.to_gtk_device(device))
    }

    fn to_gtk_device(&self, device: Arc<SoundcoreDevice>) -> Arc<GtkSoundcoreDevice> {
        Arc::new(GtkSoundcoreDevice::new(device, self.tokio_runtime.handle()))
    }
}
