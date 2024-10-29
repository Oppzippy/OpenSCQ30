use crate::{
    devices::standard::{
        packets::outbound::SetEqualizerAndCustomHearIdPacket,
        state::DeviceState,
        structures::{BasicHearId, CustomHearId, HearId, HearIdMusicType, HearIdType},
    },
    soundcore_device::device::soundcore_command::CommandResponse,
};

pub fn set_hear_id(state: DeviceState, hear_id: HearId) -> crate::Result<CommandResponse> {
    match hear_id {
        HearId::Basic(basic_hear_id) => set_basic_hear_id(state, basic_hear_id),
        HearId::Custom(custom_hear_id) => set_custom_hear_id(state, custom_hear_id),
    }
}

fn set_basic_hear_id(
    state: DeviceState,
    basic_hear_id: BasicHearId,
) -> crate::Result<CommandResponse> {
    let custom_hear_id = CustomHearId {
        is_enabled: basic_hear_id.is_enabled,
        volume_adjustments: basic_hear_id.volume_adjustments.to_owned(),
        // TODO Should this be the current time? If so, what kind of timestamp?
        time: basic_hear_id.time,
        hear_id_type: HearIdType::default(),
        hear_id_music_type: HearIdMusicType::default(),
        custom_volume_adjustments: None,
    };
    let gender = state
        .gender
        .ok_or(crate::Error::MissingData { name: "gender" })?;
    let age_range = state
        .age_range
        .ok_or(crate::Error::MissingData { name: "age range" })?;

    let packet = SetEqualizerAndCustomHearIdPacket {
        equalizer_configuration: &state.equalizer_configuration,
        gender,
        age_range,
        custom_hear_id: &custom_hear_id,
    };

    Ok(CommandResponse {
        packets: vec![packet.into()],
        new_state: DeviceState {
            hear_id: Some(HearId::Basic(basic_hear_id)),
            ..state
        },
    })
}

fn set_custom_hear_id(state: DeviceState, hear_id: CustomHearId) -> crate::Result<CommandResponse> {
    let gender = state
        .gender
        .ok_or(crate::Error::MissingData { name: "gender" })?;
    let age_range = state
        .age_range
        .ok_or(crate::Error::MissingData { name: "age range" })?;
    let packet = SetEqualizerAndCustomHearIdPacket {
        equalizer_configuration: &state.equalizer_configuration,
        gender,
        age_range,
        custom_hear_id: &hear_id,
    };

    Ok(CommandResponse {
        packets: vec![packet.into()],
        new_state: DeviceState {
            hear_id: Some(HearId::Custom(hear_id)),
            ..state
        },
    })
}
