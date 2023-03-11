use rifgen::rifgen_attr::generate_interface;

use crate::type_conversion;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SetAmbientModeOkPacket {
    packet: Option<openscq30_lib::packets::inbound::SetAmbientModeOkPacket>,
}

impl SetAmbientModeOkPacket {
    #[generate_interface(constructor)]
    pub fn new() -> Result<SetAmbientModeOkPacket, String> {
        Err("use from_bytes instead".to_string())
    }

    #[generate_interface]
    pub fn from_bytes(bytes: &[i8]) -> Result<Option<SetAmbientModeOkPacket>, String> {
        let bytes = type_conversion::i8_slice_to_u8_slice(bytes);
        Ok(
            openscq30_lib::packets::inbound::SetAmbientModeOkPacket::new(bytes)
                .map(|packet| packet.into()),
        )
    }
}

impl From<openscq30_lib::packets::inbound::SetAmbientModeOkPacket> for SetAmbientModeOkPacket {
    fn from(packet: openscq30_lib::packets::inbound::SetAmbientModeOkPacket) -> Self {
        Self {
            packet: Some(packet),
        }
    }
}
