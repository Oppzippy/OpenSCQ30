use crate::api::connection::ConnectionDescriptor;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct BtlePlugConnectionDescriptor {
    name: String,
    mac_address: String,
}

impl BtlePlugConnectionDescriptor {
    pub fn new(name: String, mac_address: String) -> Self {
        Self { name, mac_address }
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
