use std::rc::Rc;

use macaddr::MacAddr6;

use crate::api::device::{DeviceRegistry, GenericDeviceDescriptor};

use super::demo_device::DemoDevice;

#[derive(Default)]
pub struct DemoDeviceRegistry {}

impl DemoDeviceRegistry {
    const DEVICE_NAME: &'static str = "Demo Q30";
    const DEVICE_MAC_ADDRESS: MacAddr6 = MacAddr6::nil();

    pub fn new() -> Self {
        Self {}
    }
}

impl DeviceRegistry for DemoDeviceRegistry {
    type DeviceType = DemoDevice;
    type DescriptorType = GenericDeviceDescriptor;

    async fn device_descriptors(&self) -> crate::Result<Vec<Self::DescriptorType>> {
        Ok(vec![GenericDeviceDescriptor::new(
            "Demo Q30",
            MacAddr6::nil(),
        )])
    }

    async fn device(&self, mac_address: MacAddr6) -> crate::Result<Option<Rc<Self::DeviceType>>> {
        if mac_address == Self::DEVICE_MAC_ADDRESS {
            Ok(Some(Rc::new(
                DemoDevice::new(Self::DEVICE_NAME, Self::DEVICE_MAC_ADDRESS).await,
            )))
        } else {
            Ok(None)
        }
    }
}
