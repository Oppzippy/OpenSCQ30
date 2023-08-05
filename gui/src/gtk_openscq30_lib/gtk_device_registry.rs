use std::rc::Rc;

use macaddr::MacAddr6;
use openscq30_lib::api::device::DeviceRegistry;
use tokio::runtime::Runtime;

use super::{gtk_device::GtkDevice, tokio_spawn_local::TokioSpawnLocal};
use async_trait::async_trait;

pub struct GtkDeviceRegistry<InnerRegistryType>
where
    InnerRegistryType: DeviceRegistry,
{
    tokio_runtime: Rc<Runtime>,
    soundcore_device_registry: Rc<InnerRegistryType>,
}

#[async_trait(?Send)]
impl<InnerRegistryType: 'static> DeviceRegistry for GtkDeviceRegistry<InnerRegistryType>
where
    InnerRegistryType: DeviceRegistry,
{
    type DeviceType = GtkDevice<InnerRegistryType::DeviceType>;
    type DescriptorType = InnerRegistryType::DescriptorType;

    async fn device(
        &self,
        mac_address: MacAddr6,
    ) -> openscq30_lib::Result<Option<Rc<Self::DeviceType>>> {
        let mac_address = mac_address.to_owned();
        let device_registry = self.soundcore_device_registry.to_owned();
        let maybe_device = self
            .tokio_runtime
            .spawn_local(async move { device_registry.device(mac_address).await })
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
            .spawn_local(async move { device_registry.device_descriptors().await })
            .await
            .unwrap()
    }
}

impl<InnerRegistryType: 'static> GtkDeviceRegistry<InnerRegistryType>
where
    InnerRegistryType: DeviceRegistry,
{
    pub fn new(registry: InnerRegistryType, tokio_runtime: Runtime) -> Self {
        Self {
            soundcore_device_registry: Rc::new(registry),
            tokio_runtime: Rc::new(tokio_runtime),
        }
    }

    fn to_gtk_device(
        &self,
        device: Rc<InnerRegistryType::DeviceType>,
    ) -> Rc<GtkDevice<InnerRegistryType::DeviceType>> {
        Rc::new(GtkDevice::new(device, self.tokio_runtime.to_owned()))
    }
}
