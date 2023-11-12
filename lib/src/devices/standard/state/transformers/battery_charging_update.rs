use crate::devices::standard::{
    packets::inbound::BatteryChargingUpdatePacket,
    state::{DeviceState, DeviceStateTransformer},
    structures::{Battery, DualBattery, SingleBattery},
};

impl DeviceStateTransformer for BatteryChargingUpdatePacket {
    fn transform(&self, state: &DeviceState) -> DeviceState {
        DeviceState {
            battery: match state.battery {
                Battery::SingleBattery(prev_battery) => Battery::SingleBattery(SingleBattery {
                    is_charging: self.left,
                    level: prev_battery.level,
                }),
                Battery::DualBattery(prev_battery) => Battery::DualBattery(DualBattery {
                    left: SingleBattery {
                        is_charging: self.left,
                        level: prev_battery.left.level,
                    },
                    right: SingleBattery {
                        // TODO maybe switch state over to single battery if this is None
                        is_charging: self.right.unwrap_or_default(),
                        level: prev_battery.right.level,
                    },
                }),
            },
            ..state.clone()
        }
    }
}
