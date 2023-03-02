use std::hash::Hash;

use crate::api::connection::ConnectionDescriptor;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WindowsConnectionDescriptor {
    name: String,
    mac_address: String,
    address: u64,
}

impl WindowsConnectionDescriptor {
    pub fn new(name: String, mac_address: String, address: u64) -> Self {
        Self {
            name,
            mac_address,
            address,
        }
    }
}

impl ConnectionDescriptor for WindowsConnectionDescriptor {
    fn name(&self) -> &String {
        &self.name
    }

    fn mac_address(&self) -> &String {
        &self.mac_address
    }
}
