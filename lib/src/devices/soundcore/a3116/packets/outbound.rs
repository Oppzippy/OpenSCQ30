use crate::devices::soundcore::{
    a3116,
    common::{packet, structures::VolumeAdjustments},
};

pub fn set_auto_power_off(duration: &a3116::structures::AutoPowerOffDuration) -> packet::Outbound {
    packet::Outbound::new(packet::Command([0x01, 0x83]), duration.bytes().collect())
}

pub fn set_volume(volume: &a3116::structures::Volume) -> packet::Outbound {
    packet::Outbound::new(packet::Command([0x01, 0x81]), volume.bytes().collect())
}

pub fn set_equalizer_preset(preset_id: u8) -> packet::Outbound {
    packet::Outbound::new(packet::Command([0x02, 0x81]), vec![preset_id])
}

pub fn set_equalizer_volume_adjustments(
    volume_adjustments: VolumeAdjustments<9, -6, 6, 0>,
) -> packet::Outbound {
    packet::Outbound::new(
        packet::Command([0x02, 0x83]),
        volume_adjustments.bytes().to_vec(),
    )
}

pub const REQUEST_VOICE_PROMPT_COMMAND: packet::Command = packet::Command([0x01, 0x10]);
pub fn request_voice_prompt() -> packet::Outbound {
    packet::Outbound::new(REQUEST_VOICE_PROMPT_COMMAND, Vec::new())
}
