use std::sync::Arc;

use openscq30_lib::{
    api::traits::SoundcoreDeviceRegistry,
    soundcore_bluetooth::traits::SoundcoreDeviceConnectionError,
};
use tokio::runtime::Runtime;

use super::soundcore_device::GtkSoundcoreDevice;

pub struct GtkSoundcoreDeviceRegistry<RegistryType>
where
    RegistryType: SoundcoreDeviceRegistry + Send + Sync,
{
    tokio_runtime: Arc<Runtime>,
    soundcore_device_registry: Arc<RegistryType>,
}

impl<RegistryType: 'static> GtkSoundcoreDeviceRegistry<RegistryType>
where
    RegistryType: SoundcoreDeviceRegistry + Send + Sync,
{
    pub fn new(registry: RegistryType, tokio_runtime: Runtime) -> Self {
        Self {
            soundcore_device_registry: Arc::new(registry),
            tokio_runtime: Arc::new(tokio_runtime),
        }
    }

    pub async fn device(
        &self,
        mac_address: String,
    ) -> Result<
        Option<Arc<GtkSoundcoreDevice<RegistryType::DeviceType>>>,
        SoundcoreDeviceConnectionError,
    > {
        let device_registry = self.soundcore_device_registry.to_owned();
        let maybe_device = async_runtime_bridge!(
            self.tokio_runtime,
            device_registry.device(&mac_address).await
        );
        match maybe_device {
            Ok(Some(device)) => Ok(Some(self.to_gtk_device(device))),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub async fn device_descriptors(
        &self,
    ) -> Result<Vec<RegistryType::DescriptorType>, SoundcoreDeviceConnectionError> {
        let device_registry = self.soundcore_device_registry.to_owned();
        async_runtime_bridge!(
            self.tokio_runtime,
            device_registry.device_descriptors().await
        )
    }

    fn to_gtk_device(
        &self,
        device: Arc<RegistryType::DeviceType>,
    ) -> Arc<GtkSoundcoreDevice<RegistryType::DeviceType>> {
        Arc::new(GtkSoundcoreDevice::new(
            device,
            self.tokio_runtime.to_owned(),
        ))
    }
}
