use crate::api::device::DeviceDescriptor;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DemoDeviceDescriptor {
    name: String,
    mac_address: String,
}

impl DemoDeviceDescriptor {
    pub fn new(name: String, mac_address: String) -> Self {
        Self { name, mac_address }
    }
}

impl DeviceDescriptor for DemoDeviceDescriptor {
    fn name(&self) -> &String {
        &self.name
    }

    fn mac_address(&self) -> &String {
        &self.mac_address
    }
}
