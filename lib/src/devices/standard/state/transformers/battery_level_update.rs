use crate::devices::standard::{
    packets::inbound::BatteryLevelUpdatePacket,
    state::{DeviceState, DeviceStateTransformer},
    structures::{Battery, DualBattery, SingleBattery},
};

impl DeviceStateTransformer for BatteryLevelUpdatePacket {
    fn transform(&self, state: &DeviceState) -> DeviceState {
        DeviceState {
            battery: match state.battery {
                Battery::SingleBattery(prev_battery) => Battery::SingleBattery(SingleBattery {
                    is_charging: prev_battery.is_charging,
                    level: self.left,
                }),
                Battery::DualBattery(prev_battery) => Battery::DualBattery(DualBattery {
                    left: SingleBattery {
                        is_charging: prev_battery.left.is_charging,
                        level: self.left,
                    },
                    right: SingleBattery {
                        is_charging: prev_battery.right.is_charging,
                        // TODO maybe switch state over to single battery if this is None
                        level: self.right.unwrap_or_default(),
                    },
                }),
            },
            ..state.clone()
        }
    }
}
