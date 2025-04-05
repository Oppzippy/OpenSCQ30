use macaddr::MacAddr6;

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
