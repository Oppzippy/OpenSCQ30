use openscq30_lib_has::Has;

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{
        a3134::{
            self,
            structures::{A3134EqualizerConfiguration, BassUp, Lights},
        },
        common::{device::SoundcoreDeviceBuilder, modules::flag::FlagConfiguration},
    },
};

mod equalizer;
mod lights;

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<A3134EqualizerConfiguration> + Clone + Send + Sync + 'static,
{
    pub async fn a3134_equalizer(&mut self) {
        let packet_io = self.packet_io_controller().clone();
        let database = self.database();
        let device_model = self.device_model();
        let change_notify = self.change_notify();
        self.module_collection()
            .add_a3134_equalizer(packet_io, database, device_model, change_notify)
            .await;
    }
}

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<BassUp> + Clone + Send + Sync + 'static,
{
    pub fn a3134_bass_up(&mut self) {
        let packet_io = self.packet_io_controller().clone();
        self.module_collection().add_flag::<BassUp>(
            packet_io,
            FlagConfiguration {
                category_id: CategoryId::Miscellaneous,
                setting_id: SettingId::BassUp,
                set_command: a3134::packets::outbound::SET_BASS_UP_COMMAND,
                update_command: None,
                is_inverted: false,
            },
        );
    }
}

impl<StateType> SoundcoreDeviceBuilder<StateType>
where
    StateType: Has<Lights> + Clone + Send + Sync + 'static,
{
    pub fn a3134_lights(&mut self) {
        let packet_io = self.packet_io_controller().clone();
        self.module_collection().add_a3134_lights(packet_io);
    }
}
