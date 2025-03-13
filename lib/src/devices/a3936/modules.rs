use crate::{
    api::connection::RfcommConnection,
    devices::standard::{
        device::SoundcoreDeviceBuilder, packets::inbound::InboundPacket, structures::TwsStatus,
    },
};

use super::structures::A3936InternalMultiButtonConfiguration;

mod button_configuration;

impl<ConnectionType, StateType, StateUpdatePacketType>
    SoundcoreDeviceBuilder<ConnectionType, StateType, StateUpdatePacketType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateUpdatePacketType: InboundPacket + Into<StateType>,
    StateType: Send + Sync + Clone + 'static,
    StateType:
        AsRef<A3936InternalMultiButtonConfiguration> + AsMut<A3936InternalMultiButtonConfiguration>,
    StateType: AsRef<TwsStatus>,
{
    pub fn a3936_button_configuration(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3936_button_configuration(packet_io_controller);
    }
}
