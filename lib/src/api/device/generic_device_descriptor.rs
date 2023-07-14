use macaddr::MacAddr6;

use crate::api::connection::ConnectionDescriptor;

use super::DeviceDescriptor;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct GenericDeviceDescriptor {
    name: String,
    mac_address: MacAddr6,
}

impl GenericDeviceDescriptor {
    pub fn new(name: impl Into<String>, mac_address: MacAddr6) -> Self {
        Self {
            name: name.into(),
            mac_address,
        }
    }
}

impl DeviceDescriptor for GenericDeviceDescriptor {
    fn name(&self) -> &str {
        &self.name
    }

    fn mac_address(&self) -> MacAddr6 {
        self.mac_address
    }
}

impl<T> From<T> for GenericDeviceDescriptor
where
    T: ConnectionDescriptor,
{
    fn from(connection_descriptor: T) -> Self {
        Self {
            name: connection_descriptor.name().to_owned(),
            mac_address: connection_descriptor.mac_address(),
        }
    }
}
