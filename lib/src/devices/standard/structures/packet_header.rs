use serde::{Deserialize, Serialize};

use super::PacketType;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PacketHeader {
    pub packet_type: PacketType,
    pub length: u16,
}
