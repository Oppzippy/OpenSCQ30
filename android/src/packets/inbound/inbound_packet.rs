use openscq30_lib::packets::inbound::InboundPacket as LibInboundPacket;
use rifgen::rifgen_attr::generate_interface;

use crate::{
    type_conversion, SetEqualizerOkPacket, SetSoundModeOkPacket, SoundModeUpdatePacket,
    StateUpdatePacket,
};

pub struct InboundPacket(LibInboundPacket);

impl InboundPacket {
    #[generate_interface(constructor)]
    pub fn new(input: &[i8]) -> Result<InboundPacket, String> {
        let input = type_conversion::i8_slice_to_u8_slice(input);
        LibInboundPacket::new(input)
            .map(Self)
            .map_err(|err| err.to_string())
    }

    #[generate_interface]
    pub fn sound_mode_update(&self) -> Option<SoundModeUpdatePacket> {
        if let LibInboundPacket::SoundModeUpdate(packet) = &self.0 {
            Some(packet.to_owned().into())
        } else {
            None
        }
    }

    #[generate_interface]
    pub fn set_sound_mode_ok(&self) -> Option<SetSoundModeOkPacket> {
        if let LibInboundPacket::SetSoundModeOk(packet) = &self.0 {
            Some(packet.to_owned().into())
        } else {
            None
        }
    }

    #[generate_interface]
    pub fn set_equalizer_ok(&self) -> Option<SetEqualizerOkPacket> {
        if let LibInboundPacket::SetEqualizerOk(packet) = &self.0 {
            Some(packet.to_owned().into())
        } else {
            None
        }
    }

    #[generate_interface]
    pub fn state_update(&self) -> Option<StateUpdatePacket> {
        if let LibInboundPacket::StateUpdate(packet) = &self.0 {
            Some(packet.to_owned().into())
        } else {
            None
        }
    }
}
