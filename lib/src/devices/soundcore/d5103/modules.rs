use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{
        common::{device::SoundcoreDeviceBuilder, modules::flag::FlagConfiguration},
        d5103::{packets::outbound::SET_BASS_UP_COMMAND, state::D5103State, structures::BassUp},
    },
};

mod device_information;
mod equalizer;
mod lights;

impl SoundcoreDeviceBuilder<D5103State> {
    pub async fn d5103_equalizer(&mut self) {
        let packet_io = self.packet_io_controller().clone();
        let database = self.database();
        let device_model = self.device_model();
        let change_notify = self.change_notify();

        self.module_collection()
            .add_d5103_equalizer(packet_io, database, device_model, change_notify)
            .await;
    }

    pub fn d5103_bass_up(&mut self) {
        let packet_io = self.packet_io_controller().clone();
        self.module_collection().add_flag::<BassUp>(
            packet_io,
            FlagConfiguration {
                category_id: CategoryId::Miscellaneous,
                setting_id: SettingId::BassUp,
                set_command: SET_BASS_UP_COMMAND,
                update_command: None,
                is_inverted: false,
            },
        );
    }

    pub fn d5103_lights(&mut self) {
        let packet_io = self.packet_io_controller().clone();
        self.module_collection().add_d5103_lights(packet_io);
    }

    pub fn d5103_device_information(&mut self) {
        self.module_collection().add_d5103_device_information();
    }
}
