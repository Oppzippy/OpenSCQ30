use std::sync::Arc;

use async_trait::async_trait;
use futures::{stream, StreamExt};

use crate::api::device::{SoundcoreDevice, SoundcoreDeviceRegistry};

use super::{demo_soundcore_device::DemoSoundcoreDevice, DemoSoundcoreDeviceDescriptor};

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
    type DescriptorType = DemoSoundcoreDeviceDescriptor;

    async fn device_descriptors(&self) -> crate::Result<Vec<Self::DescriptorType>> {
        let descriptors = stream::iter(self.devices.iter())
            .filter_map(|device| async move {
                Some(DemoSoundcoreDeviceDescriptor::new(
                    device.name().await.unwrap(),
                    device.mac_address().await.unwrap(),
                ))
            })
            .collect::<Vec<_>>()
            .await;
        Ok(descriptors)
    }

    async fn device(&self, mac_address: &str) -> crate::Result<Option<Arc<Self::DeviceType>>> {
        let devices = stream::iter(self.devices.iter())
            .filter_map(|device| async move {
                if device.mac_address().await.unwrap() == mac_address {
                    Some(device)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .await;
        Ok(devices.first().cloned().cloned())
    }
}
