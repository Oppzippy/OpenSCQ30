use std::sync::Arc;

use async_trait::async_trait;
use mockall::mock;
use openscq30_lib::api::device::DeviceRegistry;

use super::{MockDescriptor, MockDevice};

mock! {
    pub DeviceRegistry {
        pub fn device_descriptors(&self) -> openscq30_lib::Result<Vec<MockDescriptor>>;
        pub fn device(&self, mac_address: &str) -> openscq30_lib::Result<Option<Arc<MockDevice>>>;
    }
}

#[async_trait]
impl DeviceRegistry for MockDeviceRegistry {
    type DeviceType = MockDevice;
    type DescriptorType = MockDescriptor;

    async fn device_descriptors(&self) -> openscq30_lib::Result<Vec<Self::DescriptorType>> {
        self.device_descriptors()
    }
    async fn device(
        &self,
        mac_address: &str,
    ) -> openscq30_lib::Result<Option<Arc<Self::DeviceType>>> {
        self.device(mac_address)
    }
}
