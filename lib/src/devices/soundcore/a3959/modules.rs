use crate::{
    api::connection::RfcommConnection,
    devices::soundcore::standard::{
        device::SoundcoreDeviceBuilder, packet::inbound::InboundPacket, structures::TwsStatus,
    },
};

use super::structures::{A3959MultiButtonConfiguration, A3959SoundModes};

mod button_configuration;
mod sound_modes;

impl<ConnectionType, StateType, StateUpdatePacketType>
    SoundcoreDeviceBuilder<ConnectionType, StateType, StateUpdatePacketType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateUpdatePacketType: InboundPacket + Into<StateType>,
    StateType: Send + Sync + Clone + 'static,
    StateType: AsRef<A3959MultiButtonConfiguration> + AsMut<A3959MultiButtonConfiguration>,
    StateType: AsRef<TwsStatus>,
{
    pub fn a3959_button_configuration(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3959_button_configuration(packet_io_controller);
    }
}

impl<ConnectionType, StateType, StateUpdatePacketType>
    SoundcoreDeviceBuilder<ConnectionType, StateType, StateUpdatePacketType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateUpdatePacketType: InboundPacket + Into<StateType>,
    StateType: Send + Sync + Clone + 'static,
    StateType: AsRef<A3959SoundModes> + AsMut<A3959SoundModes>,
{
    pub fn a3959_sound_modes(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3959_sound_modes(packet_io_controller);
    }
}
