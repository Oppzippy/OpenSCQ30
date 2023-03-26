use crate::api::connection::ConnectionDescriptor;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct BtlePlugConnectionDescriptor {
    name: String,
    mac_address: String,
}

impl BtlePlugConnectionDescriptor {
    pub fn new(name: impl Into<String>, mac_address: impl Into<String>) -> Self {
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

    fn mac_address(&self) -> &str {
        &self.mac_address
    }
}
