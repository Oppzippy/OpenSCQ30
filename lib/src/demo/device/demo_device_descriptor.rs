use crate::api::device::DeviceDescriptor;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DemoDeviceDescriptor {
    name: String,
    mac_address: String,
}

impl DemoDeviceDescriptor {
    pub fn new(name: impl Into<String>, mac_address: impl Into<String>) -> Self {
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

    fn mac_address(&self) -> &str {
        &self.mac_address
    }
}
