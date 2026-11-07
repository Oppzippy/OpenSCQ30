pub mod equalizer;
mod sound_modes;
mod spatial_audio;

use crate::devices::soundcore::{common::device::SoundcoreDeviceBuilder, d1205::state::D1205State};

impl SoundcoreDeviceBuilder<D1205State> {
    pub fn d1205_sound_modes(&mut self) {
        let packet_io = self.packet_io_controller().clone();
        self.module_collection().add_d1205_sound_modes(packet_io);
    }

    pub async fn d1205_equalizer(&mut self) {
        let packet_io = self.packet_io_controller().clone();
        let database = self.database();
        let device_model = self.device_model();
        let change_notify = self.change_notify();
        self.module_collection()
            .add_d1205_equalizer(packet_io, database, device_model, change_notify)
            .await;
    }

    pub fn d1205_spatial_audio(&mut self) {
        let packet_io = self.packet_io_controller().clone();
        self.module_collection().add_d1205_spatial_audio(packet_io);
    }
}
