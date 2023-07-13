use macaddr::MacAddr6;

use crate::api::connection::ConnectionDescriptor;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct BtlePlugConnectionDescriptor {
    name: String,
    mac_address: MacAddr6,
}

impl BtlePlugConnectionDescriptor {
    pub fn new(name: impl Into<String>, mac_address: MacAddr6) -> Self {
        Self {
            name: name.into(),
            mac_address: mac_address.into(),
        }
    }
}

impl ConnectionDescriptor for BtlePlugConnectionDescriptor {
    fn name(&self) -> &str {
        &self.name
    }

    fn mac_address(&self) -> MacAddr6 {
        self.mac_address
    }
}
