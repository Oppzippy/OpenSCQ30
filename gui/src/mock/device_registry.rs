use std::{rc::Rc, time::Duration};

use gtk::glib::timeout_future;
use macaddr::MacAddr6;
use mockall::mock;
use openscq30_lib::api::device::{DeviceRegistry, GenericDeviceDescriptor};

use super::MockDevice;

mock! {
    pub DeviceRegistry {
        pub fn device_descriptors(&self) -> openscq30_lib::Result<Vec<GenericDeviceDescriptor>>;
        pub fn device(&self, mac_address: MacAddr6) -> openscq30_lib::Result<Option<Rc<MockDevice>>>;
    }
}

impl DeviceRegistry for MockDeviceRegistry {
    type DeviceType = MockDevice;
    type DescriptorType = GenericDeviceDescriptor;

    async fn device_descriptors(&self) -> openscq30_lib::Result<Vec<Self::DescriptorType>> {
        timeout_future(Duration::from_millis(10)).await;
        self.device_descriptors()
    }

    async fn device(
        &self,
        mac_address: MacAddr6,
    ) -> openscq30_lib::Result<Option<Rc<Self::DeviceType>>> {
        timeout_future(Duration::from_millis(10)).await;
        self.device(mac_address)
    }
}
