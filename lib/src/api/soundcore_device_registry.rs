use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

use tokio::sync::RwLock;
use tracing::{instrument, trace, warn};

use crate::soundcore_bluetooth::traits::{
    SoundcoreDeviceConnectionError, SoundcoreDeviceConnectionRegistry,
};

use super::soundcore_device::SoundcoreDevice;

pub struct SoundcoreDeviceRegistry {
    conneciton_registry: Box<dyn SoundcoreDeviceConnectionRegistry + Send + Sync>,
    devices: RwLock<HashMap<String, Arc<SoundcoreDevice>>>,
}

impl SoundcoreDeviceRegistry {
    pub async fn new() -> Result<Self, SoundcoreDeviceConnectionError> {
        let connection_registry =
            Box::new(crate::soundcore_bluetooth::btleplug::new_handler().await?);
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

    pub async fn devices(&self) -> Vec<Arc<SoundcoreDevice>> {
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
    ) -> Option<Arc<SoundcoreDevice>> {
        self.devices.read().await.get(mac_address).cloned()
    }
}
