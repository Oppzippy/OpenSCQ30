use macaddr::MacAddr6;

use crate::devices::soundcore::common::packet::{self, Command};

pub fn request_dual_connections_devices() -> packet::Outbound {
    packet::Outbound::new(Command([0x0b, 0x01]), Vec::new())
}

pub fn set_dual_connections_enabled(is_enabled: bool) -> packet::Outbound {
    packet::Outbound::new(Command([0x0b, 0x84]), vec![is_enabled.into()])
}

pub fn dual_connections_connect(mac_address: MacAddr6) -> packet::Outbound {
    packet::Outbound::new(Command([0x0b, 0x82]), mac_address.as_bytes().to_vec())
}

pub fn dual_connections_disconnect(mac_address: MacAddr6) -> packet::Outbound {
    packet::Outbound::new(Command([0x0b, 0x81]), mac_address.as_bytes().to_vec())
}
