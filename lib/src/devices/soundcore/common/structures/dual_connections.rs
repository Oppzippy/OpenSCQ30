use macaddr::MacAddr6;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DualConnections {
    pub is_enabled: bool,
    pub devices: Vec<Option<DualConnectionsDevice>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DualConnectionsDevice {
    pub is_connected: bool,
    pub mac_address: MacAddr6,
    pub name: String,
}
