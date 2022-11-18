use crate::soundcore_bluetooth::traits::{
    soundcore_device_connection_error::SoundcoreDeviceConnectionError,
    soundcore_device_connection_registry::SoundcoreDeviceConnectionRegistry,
};

use super::soundcore_device::SoundcoreDevice;

pub struct SoundcoreDeviceRegistry {
    conneciton_registry: Box<dyn SoundcoreDeviceConnectionRegistry>,
}

impl SoundcoreDeviceRegistry {
    pub async fn new() -> Result<Self, SoundcoreDeviceConnectionError> {
        let connection_registry =
            Box::new(crate::soundcore_bluetooth::btleplug::new_handler().await?);
        Ok(Self {
            conneciton_registry: connection_registry,
        })
    }

    pub async fn refresh_devices(&mut self) -> Result<(), SoundcoreDeviceConnectionError> {
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
