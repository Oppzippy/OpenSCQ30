use std::error::Error;

use btleplug::platform::Manager;

use crate::soundcore_bluetooth::{
    btleplug::soundcore_device_connection_registry::BtlePlugSoundcoreDeviceConnectionRegistry,
    traits::soundcore_device_connection_registry::SoundcoreDeviceConnectionRegistry,
};

use super::soundcore_device::SoundcoreDevice;

pub struct SoundcoreDeviceRegistry {
    conneciton_registry: Box<dyn SoundcoreDeviceConnectionRegistry>,
}

impl SoundcoreDeviceRegistry {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let manager = Manager::new().await?;
        let connection_registry = Box::new(BtlePlugSoundcoreDeviceConnectionRegistry::new(manager));

        Ok(Self {
            conneciton_registry: connection_registry,
        })
    }

    pub async fn refresh_devices(&mut self) -> Result<(), Box<dyn Error>> {
        self.conneciton_registry.refresh_connections().await?;
        Ok(())
    }

    pub async fn get_devices(&self) -> Vec<Box<SoundcoreDevice>> {
        let connections = self.conneciton_registry.get_connections().await;
        let mut devices = Vec::new();
        for connection in connections {
            devices.push(Box::new(SoundcoreDevice::new(connection).await));
        }
        devices
    }
}
