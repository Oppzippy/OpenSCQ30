use crate::api::connection::ConnectionDescriptor;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct StubConnectionDescriptor {
    name: String,
    mac_address: String,
}

impl StubConnectionDescriptor {
    pub fn new(name: String, mac_address: String) -> Self {
        Self { name, mac_address }
    }
}

impl ConnectionDescriptor for StubConnectionDescriptor {
    fn name(&self) -> &String {
        &self.name
    }

    fn mac_address(&self) -> &String {
        &self.mac_address
    }
}