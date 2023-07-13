use macaddr::MacAddr6;

use crate::api::device::DeviceDescriptor;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DemoDeviceDescriptor {
    name: String,
    mac_address: MacAddr6,
}

impl DemoDeviceDescriptor {
    pub fn new(name: impl Into<String>, mac_address: MacAddr6) -> Self {
        Self {
            name: name.into(),
            mac_address: mac_address.into(),
        }
    }
}

impl DeviceDescriptor for DemoDeviceDescriptor {
    fn name(&self) -> &str {
        &self.name
    }

    fn mac_address(&self) -> MacAddr6 {
        self.mac_address.to_owned()
    }
}
