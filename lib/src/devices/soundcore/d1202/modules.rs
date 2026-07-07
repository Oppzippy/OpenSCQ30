mod equalizer;
mod sound_modes;
mod spatial_audio;

use crate::devices::soundcore::{
    common::{device::SoundcoreDeviceBuilder, modules::equalizer::EqualizerModuleSettings},
    d1202::state::D1202State,
};

impl SoundcoreDeviceBuilder<D1202State> {
    pub fn d1202_sound_modes(&mut self) {
        let packet_io = self.packet_io_controller().clone();
        self.module_collection().add_d1202_sound_modes(packet_io);
    }

    pub async fn d1202_equalizer<const VISIBLE_BANDS: usize, const PRESET_BANDS: usize>(
        &mut self,
        settings: EqualizerModuleSettings<VISIBLE_BANDS, PRESET_BANDS, -120, 134, 1>,
    ) {
        let packet_io = self.packet_io_controller().clone();
        let database = self.database();
        let device_model = self.device_model();
        let change_notify = self.change_notify();

        self.module_collection()
            .add_d1202_equalizer(packet_io, database, device_model, change_notify, settings)
            .await;
    }

    pub fn d1202_spatial_audio(&mut self) {
        self.module_collection().add_d1202_spatial_audio();
    }
}
