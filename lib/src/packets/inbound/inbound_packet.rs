use crate::packets::{parsing::take_packet_header, structures::PacketType};

use super::{
    take_ambient_sound_mode_update_packet, take_set_ambient_sound_mode_ok_packet,
    take_set_equalizer_ok_packet, take_state_update_packet, SetEqualizerOkPacket,
    SetSoundModeOkPacket, SoundModeUpdatePacket, StateUpdatePacket,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InboundPacket {
    StateUpdate(StateUpdatePacket),
    SoundModeUpdate(SoundModeUpdatePacket),
    SetSoundModeOk(SetSoundModeOkPacket),
    SetEqualizerOk(SetEqualizerOkPacket),
}

impl InboundPacket {
    pub fn new(input: &[u8]) -> Result<Self, nom::Err<nom::error::Error<&[u8]>>> {
        let (input, header) = take_packet_header(input)?;
        Ok(match header.packet_type {
            PacketType::SoundModeUpdate => {
                Self::SoundModeUpdate(take_ambient_sound_mode_update_packet(input)?.1)
            }
            PacketType::SetSoundModeOk => {
                Self::SetSoundModeOk(take_set_ambient_sound_mode_ok_packet(input)?.1)
            }
            PacketType::SetEqualizerOk => {
                Self::SetEqualizerOk(take_set_equalizer_ok_packet(input)?.1)
            }
            PacketType::StateUpdate => Self::StateUpdate(take_state_update_packet(input)?.1),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::InboundPacket;

    #[test]
    fn it_errors_when_nothing_matches() {
        let result = InboundPacket::new(&[1, 2, 3]);
        assert_eq!(true, result.is_err());
    }
}
