use std::sync::{Arc, Weak};

use async_trait::async_trait;
use tokio::sync::Mutex;
use weak_table::{weak_value_hash_map::Entry, WeakValueHashMap};

use crate::api::{connection::ConnectionRegistry, device::DeviceRegistry};

use super::{q30_device::Q30Device, Q30DeviceDescriptor};

pub struct Q30DeviceRegistry<RegistryType>
where
    RegistryType: ConnectionRegistry + Send + Sync,
{
    conneciton_registry: RegistryType,
    devices: Mutex<WeakValueHashMap<String, Weak<Q30Device<RegistryType::ConnectionType>>>>,
}

impl<RegistryType> Q30DeviceRegistry<RegistryType>
where
    RegistryType: ConnectionRegistry + Send + Sync,
{
    pub async fn new(connection_registry: RegistryType) -> crate::Result<Self> {
        Ok(Self {
            conneciton_registry: connection_registry,
            devices: Mutex::new(WeakValueHashMap::new()),
        })
    }

    async fn new_device(
        &self,
        mac_address: &str,
    ) -> crate::Result<Option<Q30Device<RegistryType::ConnectionType>>> {
        let connection = self.conneciton_registry.connection(mac_address).await?;

        if let Some(connection) = connection {
            Q30Device::new(connection).await.map(Option::Some)
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl<RegistryType> DeviceRegistry for Q30DeviceRegistry<RegistryType>
where
    RegistryType: ConnectionRegistry + Send + Sync,
{
    type DeviceType = Q30Device<RegistryType::ConnectionType>;
    type DescriptorType = Q30DeviceDescriptor<RegistryType::DescriptorType>;

    async fn device_descriptors(&self) -> crate::Result<Vec<Self::DescriptorType>> {
        let inner_descriptors = self.conneciton_registry.connection_descriptors().await?;
        let descriptors = inner_descriptors
            .into_iter()
            .map(|descriptor| Q30DeviceDescriptor::new(descriptor))
            .collect::<Vec<_>>();
        Ok(descriptors)
    }

    async fn device(&self, mac_address: &str) -> crate::Result<Option<Arc<Self::DeviceType>>> {
        match self.devices.lock().await.entry(mac_address.to_owned()) {
            Entry::Occupied(entry) => {
                tracing::debug!("{mac_address} is cached");
                Ok(Some(entry.get().to_owned()))
            }
            Entry::Vacant(entry) => {
                tracing::debug!("{mac_address} is not cached");
                if let Some(device) = self.new_device(mac_address).await? {
                    let device = Arc::new(device);
                    entry.insert(device.to_owned());
                    Ok(Some(device))
                } else {
                    Ok(None)
                }
            }
        }
    }
}
