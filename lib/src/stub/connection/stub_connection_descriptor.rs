use macaddr::MacAddr6;

use crate::api::connection::ConnectionDescriptor;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct StubConnectionDescriptor {
    name: String,
    mac_address: MacAddr6,
}

impl StubConnectionDescriptor {
    pub fn new(name: impl Into<String>, mac_address: MacAddr6) -> Self {
        Self {
            name: name.into(),
            mac_address,
        }
    }
}

impl ConnectionDescriptor for StubConnectionDescriptor {
    fn name(&self) -> &str {
        &self.name
    }

    fn mac_address(&self) -> MacAddr6 {
        self.mac_address.to_owned()
    }
}
