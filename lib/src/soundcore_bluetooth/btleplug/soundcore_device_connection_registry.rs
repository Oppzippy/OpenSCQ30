use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use btleplug::api::{Central, Manager as _, Peripheral as _};
use btleplug::platform::{Adapter, Manager, Peripheral};
use tracing::warn;

use crate::soundcore_bluetooth::traits::soundcore_device_connection::SoundcoreDeviceConnection;
use crate::soundcore_bluetooth::traits::soundcore_device_connection_error::SoundcoreDeviceConnectionError;
use crate::soundcore_bluetooth::traits::soundcore_device_connection_registry::SoundcoreDeviceConnectionRegistry;

use super::soundcore_device_connection::BtlePlugSoundcoreDeviceConnection;

pub struct BtlePlugSoundcoreDeviceConnectionRegistry {
    manager: Manager,
    connections: HashMap<String, Arc<dyn SoundcoreDeviceConnection + Sync + Send>>,
}

impl BtlePlugSoundcoreDeviceConnectionRegistry {
    pub fn new(manager: Manager) -> Self {
        Self {
            manager,
            connections: HashMap::new(),
        }
    }

    async fn get_soundcore_peripherals(
        &self,
        adapters: &Vec<Adapter>,
    ) -> Result<Vec<Peripheral>, SoundcoreDeviceConnectionError> {
        let mut soundcore_peripherals = Vec::new();
        for adapter in adapters {
            let peripherals = adapter
                .peripherals()
                .await
                .map_err(SoundcoreDeviceConnectionError::from)?;
            for peripheral in peripherals {
                if peripheral.is_connected().await? {
                    let is_soundcore = match peripheral.properties().await {
                        Ok(Some(properties)) => {
                            properties.address.to_string().starts_with("AC:12:2F")
                        }
                        _ => false,
                    };
                    if is_soundcore {
                        peripheral.discover_services().await?;
                        soundcore_peripherals.push(peripheral);
                    }
                }
            }
        }
        Ok(soundcore_peripherals)
    }
}

#[async_trait]
impl SoundcoreDeviceConnectionRegistry for BtlePlugSoundcoreDeviceConnectionRegistry {
    async fn refresh_connections(&mut self) -> Result<(), SoundcoreDeviceConnectionError> {
        let adapters = self
            .manager
            .adapters()
            .await
            .map_err(SoundcoreDeviceConnectionError::from)?;
        let soundcore_peripherals = self.get_soundcore_peripherals(&adapters).await?;

        for peripheral in soundcore_peripherals {
            let entry = self.connections.entry(peripheral.address().to_string());
            match entry {
                Entry::Vacant(vacant_entry) => {
                    match BtlePlugSoundcoreDeviceConnection::new(peripheral).await {
                        Ok(connection) => {
                            vacant_entry.insert(Arc::new(connection));
                        }
                        Err(err) => {
                            warn!("error creating connection: {}", err);
                        }
                    }
                }
                Entry::Occupied(_) => (),
            }
        }

        Ok(())
    }

    async fn get_connections(&self) -> Vec<Arc<dyn SoundcoreDeviceConnection + Sync + Send>> {
        self.connections
            .values()
            .into_iter()
            .map(|arc| arc.to_owned())
            .collect()
    }
}
