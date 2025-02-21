use crate::{
    devices::standard::{
        packets::outbound::{
            SetEqualizerAndCustomHearIdPacket, SetEqualizerPacket, SetEqualizerWithDrcPacket,
        },
        state::DeviceState,
        structures::{EqualizerConfiguration, HearId},
    },
    soundcore_device::device::{Packet, soundcore_command::CommandResponse},
};

pub fn set_equalizer_configuration(
    state: DeviceState,
    equalizer_configuration: EqualizerConfiguration,
) -> crate::Result<CommandResponse> {
    let left_channel = &equalizer_configuration;
    let right_channel = if state.device_features.num_equalizer_channels == 2 {
        Some(&equalizer_configuration)
    } else {
        None
    };

    let packet: Packet = if let Some(HearId::Custom(custom_hear_id)) = &state.hear_id {
        SetEqualizerAndCustomHearIdPacket {
            equalizer_configuration: &equalizer_configuration,
            age_range: state.age_range.ok_or(crate::Error::IncompleteStateError {
                message: "age range not set",
            })?,
            custom_hear_id,
            gender: state.gender.ok_or(crate::Error::IncompleteStateError {
                message: "gender not set",
            })?,
        }
        .into()
    } else if state.supports_dynamic_range_compression() {
        SetEqualizerWithDrcPacket::new(left_channel, right_channel).into()
    } else {
        SetEqualizerPacket::new(left_channel, right_channel).into()
    };
    Ok(CommandResponse {
        packets: vec![packet],
        new_state: DeviceState {
            equalizer_configuration,
            ..state
        },
    })
}
