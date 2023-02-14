use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    api::traits::{SoundcoreDevice, SoundcoreDeviceRegistry},
    soundcore_bluetooth::traits::SoundcoreDeviceConnectionError,
};

use super::demo_soundcore_device::DemoSoundcoreDevice;

pub struct DemoSoundcoreDeviceRegistry {
    devices: Vec<Arc<DemoSoundcoreDevice>>,
}

impl DemoSoundcoreDeviceRegistry {
    pub fn new() -> Self {
        Self {
            devices: vec![Arc::new(DemoSoundcoreDevice::new(
                "Demo Q30".to_string(),
                "00:00:00:00:00:00".to_string(),
            ))],
        }
    }
}

#[async_trait]
impl SoundcoreDeviceRegistry for DemoSoundcoreDeviceRegistry {
    type DeviceType = DemoSoundcoreDevice;

    async fn refresh_devices(&self) -> Result<(), SoundcoreDeviceConnectionError> {
        Ok(())
    }

    async fn devices(&self) -> Vec<Arc<Self::DeviceType>> {
        self.devices.to_owned()
    }

    async fn device_by_mac_address(&self, mac_address: &String) -> Option<Arc<Self::DeviceType>> {
        for device in self.devices.iter() {
            if &device.mac_address().await.unwrap() == mac_address {
                return Some(device.to_owned());
            }
        }
        None
    }
}
