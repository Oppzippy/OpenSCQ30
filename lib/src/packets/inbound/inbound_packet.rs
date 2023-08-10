use nom::error::VerboseError;

use crate::packets::{
    parsing::{take_checksum, take_packet_header},
    structures::PacketType,
};

use super::{
    state_update_packet::{take_state_update_packet, StateUpdatePacket},
    take_ambient_sound_mode_update_packet, take_firmware_version_update_packet,
    take_set_ambient_sound_mode_ok_packet, take_set_equalizer_ok_packet,
    FirmwareVersionUpdatePacket, SetEqualizerOkPacket, SetSoundModeOkPacket, SoundModeUpdatePacket,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InboundPacket {
    StateUpdate(StateUpdatePacket),
    SoundModeUpdate(SoundModeUpdatePacket),
    SetSoundModeOk(SetSoundModeOkPacket),
    SetEqualizerOk(SetEqualizerOkPacket),
    FirmwareVersionUpdate(FirmwareVersionUpdatePacket),
}

impl InboundPacket {
    pub fn new(input: &[u8]) -> Result<Self, nom::Err<VerboseError<&[u8]>>> {
        let input = take_checksum(input)?.0;
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
            PacketType::FirmwareVersionUpdate => {
                Self::FirmwareVersionUpdate(take_firmware_version_update_packet(input)?.1)
            }
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
