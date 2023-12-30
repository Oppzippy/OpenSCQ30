use nom::{combinator::all_consuming, error::VerboseError};

use crate::devices::standard::{
    packets::inbound::take_ambient_sound_mode_update_packet, state::DeviceState,
    structures::SoundModes,
};

pub fn sound_mode_update_handler(input: &[u8], state: DeviceState) -> DeviceState {
    let result: Result<_, nom::Err<VerboseError<&[u8]>>> =
        all_consuming(take_ambient_sound_mode_update_packet)(input);
    let packet = match result {
        Ok((_, packet)) => packet,
        Err(err) => {
            tracing::error!("failed to parse packet: {err:?}");
            return state;
        }
    };
    DeviceState {
        sound_modes: Some(SoundModes {
            ambient_sound_mode: packet.ambient_sound_mode,
            noise_canceling_mode: packet.noise_canceling_mode,
            custom_noise_canceling: packet.custom_noise_canceling,
            transparency_mode: packet.transparency_mode,
        }),
        ..state.clone()
    }
}
