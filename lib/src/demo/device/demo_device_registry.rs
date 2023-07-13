use std::sync::Arc;

use async_trait::async_trait;
use macaddr::MacAddr6;

use crate::api::device::DeviceRegistry;

use super::{demo_device::DemoDevice, DemoDeviceDescriptor};

#[derive(Default)]
pub struct DemoDeviceRegistry {}

impl DemoDeviceRegistry {
    const DEVICE_NAME: &str = "Demo Q30";
    const DEVICE_MAC_ADDRESS: MacAddr6 = MacAddr6::nil();

    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl DeviceRegistry for DemoDeviceRegistry {
    type DeviceType = DemoDevice;
    type DescriptorType = DemoDeviceDescriptor;

    async fn device_descriptors(&self) -> crate::Result<Vec<Self::DescriptorType>> {
        Ok(vec![DemoDeviceDescriptor::new("Demo Q30", MacAddr6::nil())])
    }

    async fn device(&self, mac_address: MacAddr6) -> crate::Result<Option<Arc<Self::DeviceType>>> {
        if mac_address == Self::DEVICE_MAC_ADDRESS {
            Ok(Some(Arc::new(
                DemoDevice::new(Self::DEVICE_NAME, Self::DEVICE_MAC_ADDRESS).await,
            )))
        } else {
            Ok(None)
        }
    }
}
