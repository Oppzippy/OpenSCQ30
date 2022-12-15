use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

use tokio::sync::RwLock;
use tracing::{instrument, trace, warn};

use crate::soundcore_bluetooth::traits::{
    SoundcoreDeviceConnection, SoundcoreDeviceConnectionError, SoundcoreDeviceConnectionRegistry,
};

use super::soundcore_device::SoundcoreDevice;

pub struct SoundcoreDeviceRegistry<RegistryType: 'static>
where
    RegistryType: SoundcoreDeviceConnectionRegistry + Send + Sync,
{
    conneciton_registry: RegistryType,
    devices: RwLock<HashMap<String, Arc<SoundcoreDevice<RegistryType::DeviceConnectionType>>>>,
}

impl<RegistryType: 'static> SoundcoreDeviceRegistry<RegistryType>
where
    RegistryType: SoundcoreDeviceConnectionRegistry + Send + Sync,
{
    pub async fn new(
        connection_registry: RegistryType,
    ) -> Result<Self, SoundcoreDeviceConnectionError> {
        Ok(Self {
            conneciton_registry: connection_registry,
            devices: RwLock::new(HashMap::new()),
        })
    }

    #[instrument(level = "trace", skip(self))]
    pub async fn refresh_devices(&self) -> Result<(), SoundcoreDeviceConnectionError> {
        self.conneciton_registry.refresh_connections().await?;
        let connections = self.conneciton_registry.connections().await;

        let mut devices = self.devices.write().await;
        for connection in connections {
            let mac_address = connection.mac_address().await?;
            match devices.entry(mac_address.to_owned()) {
                Entry::Vacant(entry) => match SoundcoreDevice::new(connection).await {
                    Ok(device) => {
                        entry.insert(Arc::new(device));
                        trace!("added new device: {mac_address}");
                    }
                    Err(err) => warn!("failed to initialize soundcore device: {}", err),
                },
                Entry::Occupied(_) => {
                    trace!("found existing device: {mac_address}")
                }
            }
        }
        Ok(())
    }

    pub async fn devices(&self) -> Vec<Arc<SoundcoreDevice<RegistryType::DeviceConnectionType>>> {
        self.devices
            .read()
            .await
            .values()
            .map(|x| x.to_owned())
            .collect()
    }

    pub async fn device_by_mac_address(
        &self,
        mac_address: &String,
    ) -> Option<Arc<SoundcoreDevice<RegistryType::DeviceConnectionType>>> {
        self.devices.read().await.get(mac_address).cloned()
    }
}
