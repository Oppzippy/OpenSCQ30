use rifgen::rifgen_attr::generate_interface;

use crate::type_conversion;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct OkPacket {
    packet: Option<openscq30_lib::packets::inbound::OkPacket>,
}

impl OkPacket {
    #[generate_interface(constructor)]
    pub fn new() -> Result<OkPacket, String> {
        Err("use from_bytes instead".to_string())
    }

    #[generate_interface]
    pub fn from_bytes(bytes: &[i16]) -> Result<Option<OkPacket>, String> {
        let bytes = type_conversion::i16_slice_to_u8_vec(bytes).map_err(|err| err.to_string())?;
        Ok(openscq30_lib::packets::inbound::OkPacket::new(&bytes).map(|packet| packet.into()))
    }
}

impl From<openscq30_lib::packets::inbound::OkPacket> for OkPacket {
    fn from(packet: openscq30_lib::packets::inbound::OkPacket) -> Self {
        Self {
            packet: Some(packet),
        }
    }
}
