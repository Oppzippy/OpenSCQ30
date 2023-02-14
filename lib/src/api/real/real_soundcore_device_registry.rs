use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

use async_trait::async_trait;
use tokio::sync::RwLock;
use tracing::{instrument, trace, warn};

use crate::{
    api::traits::SoundcoreDeviceRegistry,
    soundcore_bluetooth::traits::{
        SoundcoreDeviceConnection, SoundcoreDeviceConnectionError,
        SoundcoreDeviceConnectionRegistry,
    },
};

use super::real_soundcore_device::RealSoundcoreDevice;

pub struct RealSoundcoreDeviceRegistry<RegistryType: 'static>
where
    RegistryType: SoundcoreDeviceConnectionRegistry + Send + Sync,
{
    conneciton_registry: RegistryType,
    devices: RwLock<HashMap<String, Arc<RealSoundcoreDevice<RegistryType::DeviceConnectionType>>>>,
}

impl<RegistryType: 'static> RealSoundcoreDeviceRegistry<RegistryType>
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
}

#[async_trait]
impl<RegistryType: 'static> SoundcoreDeviceRegistry for RealSoundcoreDeviceRegistry<RegistryType>
where
    RegistryType: SoundcoreDeviceConnectionRegistry + Send + Sync,
{
    type DeviceType = RealSoundcoreDevice<RegistryType::DeviceConnectionType>;

    #[instrument(level = "trace", skip(self))]
    async fn refresh_devices(&self) -> Result<(), SoundcoreDeviceConnectionError> {
        self.conneciton_registry.refresh_connections().await?;
        let connections = self.conneciton_registry.connections().await;

        let mut devices = self.devices.write().await;
        for connection in connections {
            let mac_address = connection.mac_address().await?;
            match devices.entry(mac_address.to_owned()) {
                Entry::Vacant(entry) => match RealSoundcoreDevice::new(connection).await {
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

    async fn devices(&self) -> Vec<Arc<RealSoundcoreDevice<RegistryType::DeviceConnectionType>>> {
        self.devices
            .read()
            .await
            .values()
            .map(|x| x.to_owned())
            .collect()
    }

    async fn device_by_mac_address(
        &self,
        mac_address: &String,
    ) -> Option<Arc<RealSoundcoreDevice<RegistryType::DeviceConnectionType>>> {
        self.devices.read().await.get(mac_address).cloned()
    }
}
