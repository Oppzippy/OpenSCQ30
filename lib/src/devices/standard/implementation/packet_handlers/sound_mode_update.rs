use nom::{combinator::all_consuming, error::VerboseError};

use crate::devices::standard::{
    packets::inbound::{InboundPacket, SoundModeUpdatePacket},
    state::DeviceState,
};

pub fn sound_mode_update_handler(input: &[u8], state: DeviceState) -> DeviceState {
    let result: Result<_, nom::Err<VerboseError<&[u8]>>> =
        all_consuming(SoundModeUpdatePacket::take)(input);
    let packet = match result {
        Ok((_, packet)) => packet,
        Err(err) => {
            tracing::error!("failed to parse packet: {err:?}");
            return state;
        }
    };
    DeviceState {
        sound_modes: Some(packet.0),
        ..state.clone()
    }
}
