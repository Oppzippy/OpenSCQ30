use std::sync::Arc;

use async_trait::async_trait;
use futures::{stream, StreamExt};

use crate::api::device::{Device, DeviceRegistry};

use super::{demo_device::DemoDevice, DemoDeviceDescriptor};

pub struct DemoDeviceRegistry {
    devices: Vec<Arc<DemoDevice>>,
}

impl DemoDeviceRegistry {
    pub fn new() -> Self {
        Self {
            devices: vec![Arc::new(DemoDevice::new(
                "Demo Q30".to_string(),
                "00:00:00:00:00:00".to_string(),
            ))],
        }
    }
}

#[async_trait]
impl DeviceRegistry for DemoDeviceRegistry {
    type DeviceType = DemoDevice;
    type DescriptorType = DemoDeviceDescriptor;

    async fn device_descriptors(&self) -> crate::Result<Vec<Self::DescriptorType>> {
        let descriptors = stream::iter(self.devices.iter())
            .filter_map(|device| async move {
                Some(DemoDeviceDescriptor::new(
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
