use crate::packets::inbound::InboundPacket;

use super::DeviceState;

pub trait DeviceStateTransformer {
    fn transform(&self, state: &DeviceState) -> DeviceState;
}

pub fn transform_state(value: InboundPacket, state: &DeviceState) -> DeviceState {
    match value {
        InboundPacket::StateUpdate(packet) => packet.transform(state),
        InboundPacket::SoundModeUpdate(packet) => packet.transform(state),
        InboundPacket::FirmwareVersionUpdate(packet) => packet.transform(state),
        InboundPacket::BatteryLevelUpdate(packet) => packet.transform(state),
        InboundPacket::BatteryChargingUpdate(packet) => packet.transform(state),
        InboundPacket::SetSoundModeOk(_packet) => state.to_owned(),
        InboundPacket::SetEqualizerOk(_packet) => state.to_owned(),
        InboundPacket::SetEqualizerWithDrcOk(_packet) => state.to_owned(),
    }
}
