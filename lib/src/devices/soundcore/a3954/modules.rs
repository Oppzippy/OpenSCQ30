use crate::devices::soundcore::{
    a3954::state::A3954State,
    common::{device::SoundcoreDeviceBuilder, modules::equalizer::EqualizerModuleSettings},
};

mod air_pressure;
mod case_features;
mod case_language;
mod case_serial_number_and_firmware_version;
mod easy_chat;
mod equalizer;
mod sound_modes;
mod spatial_audio;

impl SoundcoreDeviceBuilder<A3954State> {
    pub fn a3954_sound_modes(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3954_sound_modes(packet_io_controller);
    }

    pub fn a3954_case_features(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3954_case_features(packet_io_controller);
    }

    pub fn a3954_case_language(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3954_case_language(packet_io_controller);
    }

    pub fn a3954_case_serial_number_and_firmware_version(&mut self) {
        self.module_collection()
            .add_a3954_case_serial_number_and_firmware_version();
    }

    pub fn a3954_air_pressure(&mut self) {
        self.module_collection().add_a3954_air_pressure();
    }

    pub fn a3954_easy_chat(&mut self) {
        let packet_io_controller = self.packet_io_controller().clone();
        self.module_collection()
            .add_a3954_easy_chat(packet_io_controller);
    }

    pub fn a3954_spatial_audio(&mut self) {
        self.module_collection().add_a3954_spatial_audio();
    }

    pub async fn a3954_equalizer<const VISIBLE_BANDS: usize, const PRESET_BANDS: usize>(
        &mut self,
        settings: EqualizerModuleSettings<VISIBLE_BANDS, PRESET_BANDS, -120, 134, 1>,
    ) {
        let packet_io = self.packet_io_controller().clone();
        let database = self.database();
        let device_model = self.device_model();
        let change_notify = self.change_notify();

        self.module_collection()
            .add_a3954_equalizer(packet_io, database, device_model, change_notify, settings)
            .await;
    }
}
