use macaddr::MacAddr6;

use crate::api::device::DeviceDescriptor;

use super::ConnectionDescriptor;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct GenericConnectionDescriptor {
    name: String,
    mac_address: MacAddr6,
}

impl GenericConnectionDescriptor {
    pub fn new(name: impl Into<String>, mac_address: MacAddr6) -> Self {
        Self {
            name: name.into(),
            mac_address,
        }
    }
}

impl ConnectionDescriptor for GenericConnectionDescriptor {
    fn name(&self) -> &str {
        &self.name
    }

    fn mac_address(&self) -> MacAddr6 {
        self.mac_address
    }
}

impl<T> From<T> for GenericConnectionDescriptor
where
    T: DeviceDescriptor,
{
    fn from(device_descriptor: T) -> Self {
        GenericConnectionDescriptor {
            name: device_descriptor.name().to_owned(),
            mac_address: device_descriptor.mac_address(),
        }
    }
}
