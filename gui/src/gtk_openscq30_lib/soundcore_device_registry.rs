use std::sync::Arc;

use openscq30_lib::api::soundcore_device_registry::SoundcoreDeviceRegistry;
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

    pub async fn get_devices(&self) -> Vec<Arc<GtkSoundcoreDevice>> {
        let device_registry = self.soundcore_device_registry.to_owned();
        let devices =
            async_runtime_bridge!(self.tokio_runtime, device_registry.get_devices().await);
        devices
            .into_iter()
            .map(|device| Arc::new(GtkSoundcoreDevice::new(device, self.tokio_runtime.handle())))
            .collect()
    }
}
