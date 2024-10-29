use crate::{
    devices::standard::{
        packets::outbound::SetAmbientSoundModeCyclePacket, state::DeviceState,
        structures::AmbientSoundModeCycle,
    },
    soundcore_device::device::soundcore_command::CommandResponse,
};

pub fn set_ambient_sound_mode_cycle(
    state: DeviceState,
    cycle: AmbientSoundModeCycle,
) -> crate::Result<CommandResponse> {
    if !state.device_features.has_ambient_sound_mode_cycle {
        return Err(crate::Error::FeatureNotSupported {
            feature_name: "ambient sound mode cycle",
        });
    }

    let packet = SetAmbientSoundModeCyclePacket { cycle };
    Ok(CommandResponse {
        packets: vec![packet.into()],
        new_state: DeviceState {
            ambient_sound_mode_cycle: Some(cycle),
            ..state
        },
    })
}
