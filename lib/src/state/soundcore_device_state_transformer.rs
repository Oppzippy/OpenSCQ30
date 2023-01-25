use crate::packets::inbound::InboundPacket;

use super::SoundcoreDeviceState;

pub trait SoundcoreDeviceStateTransformer {
    fn transform(&self, state: &SoundcoreDeviceState) -> SoundcoreDeviceState;
}

pub fn inbound_packet_to_state_transformer(
    value: InboundPacket,
) -> Option<Box<dyn SoundcoreDeviceStateTransformer + Send + Sync>> {
    match value {
        InboundPacket::StateUpdate(packet) => Some(Box::new(packet)),
        InboundPacket::AmbientSoundModeUpdate(packet) => Some(Box::new(packet)),
        InboundPacket::SetAmbientModeOk(_packet) => None,
        InboundPacket::SetEqualizerOk(_packet) => None,
    }
}
