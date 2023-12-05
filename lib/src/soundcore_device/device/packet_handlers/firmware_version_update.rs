use nom::{combinator::all_consuming, error::VerboseError};

use crate::devices::standard::{
    packets::inbound::take_firmware_version_update_packet, state::DeviceState,
};

pub fn firmware_version_update_handler(input: &[u8], state: DeviceState) -> DeviceState {
    let result: Result<_, nom::Err<VerboseError<&[u8]>>> =
        all_consuming(take_firmware_version_update_packet)(&input);
    let packet = match result {
        Ok((_, packet)) => packet,
        Err(err) => {
            tracing::error!("failed to parse packet: {err:?}");
            return state;
        }
    };
    DeviceState {
        firmware_version: Some(
            packet
                .left_firmware_version
                .max(packet.right_firmware_version),
        ),
        ..state.clone()
    }
}
