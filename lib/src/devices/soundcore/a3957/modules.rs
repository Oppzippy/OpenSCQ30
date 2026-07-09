use crate::devices::soundcore::{
    a3957::state::A3957State,
    common::{device::SoundcoreDeviceBuilder, modules::equalizer::EqualizerModuleSettings},
};

mod equalizer;
mod flag;
mod sound_modes;

impl SoundcoreDeviceBuilder<A3957State> {
    pub fn a3957_sound_modes(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3957_sound_modes(packet_io_controller);
    }

    pub async fn a3957_equalizer<const VISIBLE_BANDS: usize, const PRESET_BANDS: usize>(
        &mut self,
        settings: EqualizerModuleSettings<VISIBLE_BANDS, PRESET_BANDS, -120, 134, 1>,
    ) {
        let packet_io = self.packet_io_controller().clone();
        let database = self.database();
        let device_model = self.device_model();
        let change_notify = self.change_notify();

        self.module_collection()
            .add_a3957_equalizer(packet_io, database, device_model, change_notify, settings)
            .await;
    }

    pub fn a3957_gaming_mode(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3957_gaming_mode(packet_io_controller);
    }
}
