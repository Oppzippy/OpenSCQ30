use std::sync::Arc;

use macaddr::MacAddr6;
use openscq30_lib::api::device::DeviceRegistry;
use tokio::runtime::Runtime;

use super::gtk_device::GtkDevice;
use async_trait::async_trait;

pub struct GtkDeviceRegistry<InnerRegistryType>
where
    InnerRegistryType: DeviceRegistry + Send + Sync,
{
    tokio_runtime: Arc<Runtime>,
    soundcore_device_registry: Arc<InnerRegistryType>,
}

#[async_trait]
impl<InnerRegistryType: 'static> DeviceRegistry for GtkDeviceRegistry<InnerRegistryType>
where
    InnerRegistryType: DeviceRegistry + Send + Sync,
{
    type DeviceType = GtkDevice<InnerRegistryType::DeviceType>;
    type DescriptorType = InnerRegistryType::DescriptorType;

    async fn device(
        &self,
        mac_address: MacAddr6,
    ) -> openscq30_lib::Result<Option<Arc<Self::DeviceType>>> {
        let mac_address = mac_address.to_owned();
        let device_registry = self.soundcore_device_registry.to_owned();
        let maybe_device = self
            .tokio_runtime
            .spawn(async move { device_registry.device(mac_address).await })
            .await
            .unwrap();
        match maybe_device {
            Ok(Some(device)) => Ok(Some(self.to_gtk_device(device))),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    async fn device_descriptors(&self) -> openscq30_lib::Result<Vec<Self::DescriptorType>> {
        let device_registry = self.soundcore_device_registry.to_owned();
        self.tokio_runtime
            .spawn(async move { device_registry.device_descriptors().await })
            .await
            .unwrap()
    }
}

impl<InnerRegistryType: 'static> GtkDeviceRegistry<InnerRegistryType>
where
    InnerRegistryType: DeviceRegistry + Send + Sync,
{
    pub fn new(registry: InnerRegistryType, tokio_runtime: Runtime) -> Self {
        Self {
            soundcore_device_registry: Arc::new(registry),
            tokio_runtime: Arc::new(tokio_runtime),
        }
    }

    fn to_gtk_device(
        &self,
        device: Arc<InnerRegistryType::DeviceType>,
    ) -> Arc<GtkDevice<InnerRegistryType::DeviceType>> {
        Arc::new(GtkDevice::new(device, self.tokio_runtime.to_owned()))
    }
}
