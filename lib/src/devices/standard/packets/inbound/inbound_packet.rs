use nom::error::VerboseError;

use crate::devices::standard::{
    packets::parsing::{take_checksum, take_packet_header},
    structures::PacketType,
};

use super::{
    state_update_packet::{take_state_update_packet, StateUpdatePacket},
    take_ambient_sound_mode_update_packet, take_battery_charging_update_packet,
    take_battery_level_update_packet, take_chinese_voice_prompt_state_update_packet,
    take_firmware_version_update_packet, take_ldac_state_update_packet,
    take_set_ambient_sound_mode_ok_packet, take_set_equalizer_ok_packet,
    take_set_equalizer_with_drc_ok_packet, take_tws_status_update_packet,
    BatteryChargingUpdatePacket, BatteryLevelUpdatePacket, ChineseVoicePromptStateUpdatePacket,
    FirmwareVersionUpdatePacket, LdacStateUpdatePacket, SetEqualizerOkPacket,
    SetEqualizerWithDrcOkPacket, SetSoundModeOkPacket, SoundModeUpdatePacket,
    TwsStatusUpdatePacket,
};

#[derive(Debug, Clone, PartialEq)]
pub enum InboundPacket {
    StateUpdate(StateUpdatePacket),
    SoundModeUpdate(SoundModeUpdatePacket),
    SetSoundModeOk(SetSoundModeOkPacket),
    SetEqualizerOk(SetEqualizerOkPacket),
    SetEqualizerWithDrcOk(SetEqualizerWithDrcOkPacket),
    FirmwareVersionUpdate(FirmwareVersionUpdatePacket),
    BatteryLevelUpdate(BatteryLevelUpdatePacket),
    BatteryChargingUpdate(BatteryChargingUpdatePacket),
    TwsStatusUpdate(TwsStatusUpdatePacket),
    LdacStateUpdate(LdacStateUpdatePacket),
    ChineseVoicePromptStateUpdate(ChineseVoicePromptStateUpdatePacket),
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
            PacketType::SetEqualizerWithDrcOk => {
                Self::SetEqualizerWithDrcOk(take_set_equalizer_with_drc_ok_packet(input)?.1)
            }
            PacketType::StateUpdate => Self::StateUpdate(take_state_update_packet(input)?.1),
            PacketType::FirmwareVersionUpdate => {
                Self::FirmwareVersionUpdate(take_firmware_version_update_packet(input)?.1)
            }
            PacketType::BatteryLevelUpdate => {
                Self::BatteryLevelUpdate(take_battery_level_update_packet(input)?.1)
            }
            PacketType::BatteryChargingUpdate => {
                Self::BatteryChargingUpdate(take_battery_charging_update_packet(input)?.1)
            }
            PacketType::TwsStatusUpdate => {
                Self::TwsStatusUpdate(take_tws_status_update_packet(input)?.1)
            }
            PacketType::LdacStateUpdate => {
                Self::LdacStateUpdate(take_ldac_state_update_packet(input)?.1)
            }
            PacketType::ChineseVoicePromptStateUpdate => Self::ChineseVoicePromptStateUpdate(
                take_chinese_voice_prompt_state_update_packet(input)?.1,
            ),
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
