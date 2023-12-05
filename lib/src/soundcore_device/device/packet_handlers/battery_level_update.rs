use nom::{combinator::all_consuming, error::VerboseError};

use crate::devices::standard::{
    packets::inbound::take_battery_level_update_packet,
    state::DeviceState,
    structures::{Battery, DualBattery, SingleBattery},
};

pub fn battery_level_update_handler(input: &[u8], state: DeviceState) -> DeviceState {
    let result: Result<_, nom::Err<VerboseError<&[u8]>>> =
        all_consuming(take_battery_level_update_packet)(&input);
    let packet = match result {
        Ok((_, packet)) => packet,
        Err(err) => {
            tracing::error!("failed to parse packet: {err:?}");
            return state;
        }
    };
    DeviceState {
        battery: match state.battery {
            Battery::SingleBattery(prev_battery) => Battery::SingleBattery(SingleBattery {
                is_charging: prev_battery.is_charging,
                level: packet.left,
            }),
            Battery::DualBattery(prev_battery) => Battery::DualBattery(DualBattery {
                left: SingleBattery {
                    is_charging: prev_battery.left.is_charging,
                    level: packet.left,
                },
                right: SingleBattery {
                    is_charging: prev_battery.right.is_charging,
                    // TODO maybe switch state over to single battery if this is None
                    level: packet.right.unwrap_or_default(),
                },
            }),
        },
        ..state.clone()
    }
}
