use std::sync::Arc;

use openscq30_lib::api::{
    soundcore_device::SoundcoreDevice, soundcore_device_registry::SoundcoreDeviceRegistry,
};
use tokio::runtime::{Handle, Runtime};

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

    pub async fn get_devices(self) -> Vec<Box<SoundcoreDevice>> {
        async_runtime_bridge!(
            self.tokio_runtime,
            self.soundcore_device_registry.get_devices().await,
        )
    }
}
