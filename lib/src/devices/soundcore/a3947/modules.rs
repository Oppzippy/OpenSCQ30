use openscq30_lib_has::Has;

use crate::{
    api::connection::RfcommConnection,
    devices::soundcore::{
        a3947,
        common::{
            device::SoundcoreDeviceBuilder,
            packet::inbound::FromPacketBody,
            structures::{EqualizerConfiguration, TwsStatus},
        },
    },
};

use super::structures::SoundModes;

mod equalizer;
mod sound_modes;

impl<ConnectionType, StateType, StateUpdatePacketType>
    SoundcoreDeviceBuilder<ConnectionType, StateType, StateUpdatePacketType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateUpdatePacketType: FromPacketBody + Into<StateType>,
    StateType: Has<SoundModes> + Send + Sync + Clone + 'static,
{
    pub fn a3947_sound_modes(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3947_sound_modes(packet_io_controller);
    }
}

impl<ConnectionType, StateType, StateUpdatePacketType>
    SoundcoreDeviceBuilder<ConnectionType, StateType, StateUpdatePacketType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateUpdatePacketType: FromPacketBody + Into<StateType>,
    StateType: Has<EqualizerConfiguration<2, 10>>
        + Has<a3947::structures::HearId<2, 10>>
        + Has<TwsStatus>
        + Send
        + Sync
        + Clone
        + 'static,
{
    pub async fn a3947_equalizer(&mut self) {
        let database = self.database();
        let device_model = self.device_model();
        let change_notify = self.change_notify();
        let packet_io = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3947_equalizer(database, device_model, change_notify, packet_io)
            .await;
    }
}
