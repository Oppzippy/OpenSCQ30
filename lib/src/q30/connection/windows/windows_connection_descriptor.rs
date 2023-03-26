use std::hash::Hash;

use crate::api::connection::ConnectionDescriptor;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WindowsConnectionDescriptor {
    name: String,
    mac_address: String,
}

impl WindowsConnectionDescriptor {
    pub fn new(name: impl Into<String>, mac_address: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            mac_address: mac_address.into(),
        }
    }
}

impl ConnectionDescriptor for WindowsConnectionDescriptor {
    fn name(&self) -> &str {
        &self.name
    }

    fn mac_address(&self) -> &str {
        &self.mac_address
    }
}
